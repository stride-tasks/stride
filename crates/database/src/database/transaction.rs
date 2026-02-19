use indoc::{concatdoc, indoc};
use rusqlite::OptionalExtension;
use stride_core::task::Task;
use stride_crdt::{
    actor::{Actor, ActorId},
    change::{
        AnnotationOperation, Change, Operation, OperationKind, Sequence, TaskOperation,
        serialize::{operation_from_data, operation_to_data, operation_type},
    },
    difference::push_task_diff_operations,
    hlc::{Clock, Microsecond, Timestamp},
    version_vector::{ChangeRange, VersionVector},
};
use uuid::Uuid;

use crate::{
    Database, Error, Result,
    conversion::{Sql, task_priority_to_sql, task_status_to_sql},
    database::SQL_BY_ID,
};

macro_rules! update_sql {
    ($table:expr, $field:expr) => {
        concatdoc! {"
            INSERT INTO ", $table, "(id, ", $field,")
            VALUES (?1, ?2)
            ON CONFLICT(id) DO UPDATE
            SET
                ", $field, " = excluded.", $field, "
        "}
    };
}

#[derive(Debug)]
pub struct Transaction<'a> {
    actor_id: ActorId,
    clock: &'a mut Clock,
    version_vector: VersionVector,
    #[allow(clippy::struct_field_names)]
    transaction: rusqlite::Transaction<'a>,
}

impl<'a> Transaction<'a> {
    pub fn new(db: &'a mut Database) -> Result<Transaction<'a>> {
        let transaction = db.connection.transaction()?;
        let mut sql = transaction.prepare(indoc! {"
                SELECT
                    id,
                    sequence,
                    timestamp_logical,
                    timestamp_counter
                FROM
                    actor_table
            "})?;
        let rows = sql.query_map((), |row| {
            let id = row.get::<_, Uuid>("id")?;
            let sequence = row.get::<_, u64>("sequence")?;
            let timestamp_logical = row.get::<_, i64>("timestamp_logical")?;
            let timestamp_counter = row.get::<_, u32>("timestamp_counter")?;

            Ok(Actor {
                id: ActorId::new(id),
                sequence: Sequence::new(sequence),
                timestamp: Timestamp::new(Microsecond::new(timestamp_logical), timestamp_counter),
            })
        })?;

        let mut version_vector = VersionVector::default();
        for row in rows {
            version_vector.insert(row?);
        }
        drop(sql);

        let mut this = Self {
            actor_id: db.actor_id,
            clock: &mut db.clock,
            version_vector,
            transaction,
        };

        this.get_or_insert_actor(db.actor_id)?;

        Ok(this)
    }

    pub fn get_or_insert_actor(&mut self, actor_id: ActorId) -> Result<Actor> {
        if !self.version_vector.contains(actor_id) {
            let timestamp = self.clock.tick()?;
            let actor = Actor::new(actor_id, timestamp);
            self.transaction.execute(
                indoc! {"
                INSERT INTO actor_table
                    (id, timestamp_logical, timestamp_counter)
                VALUES
                    (?1, 0, 0)
            "},
                (actor_id.get(),),
            )?;
            self.version_vector.insert(actor.clone());
            return Ok(actor);
        }

        Ok(self.version_vector.get(actor_id).cloned()?)
    }

    pub fn version_vector(&self) -> &VersionVector {
        &self.version_vector
    }

    pub fn actor_id(&self) -> ActorId {
        self.actor_id
    }

    pub fn insert_task(&mut self, task: &Task) -> Result<()> {
        let mut operations = Vec::new();
        push_task_diff_operations(task.id, task, None, &mut operations);

        let actor_id = self.actor_id;
        let sequence = self.version_vector.next_sequence(actor_id)?;
        let timestamp = self.clock.timestamp();
        let change = Change {
            actor_id,
            sequence,
            timestamp,
            operations,
        };
        self.apply_change(&change)?;
        Ok(())
    }

    pub fn delete_task(&mut self, id: Uuid) -> Result<()> {
        let actor_id = self.actor_id;
        let sequence = self.version_vector.next_sequence(actor_id)?;
        let timestamp = self.clock.timestamp();
        let change = Change {
            actor_id,
            sequence,
            timestamp,
            operations: vec![Operation {
                row_id: id.into(),
                kind: TaskOperation::Delete.into(),
            }],
        };
        self.apply_change(&change)?;
        Ok(())
    }

    pub fn task_by_id(&mut self, id: Uuid) -> Result<Option<Task>> {
        self.transaction
            .query_row(SQL_BY_ID, (id,), Database::row_to_task)
            .optional()
            .map_err(Into::into)
    }

    pub fn update_task_with(
        &mut self,
        id: Uuid,
        f: impl FnOnce(Task) -> Result<Task>,
    ) -> Result<Task> {
        let Some(task) = self.task_by_id(id)? else {
            return Err(Error::TaskNotFound { id });
        };

        let updated = f(task.clone())?;

        let mut operations = Vec::new();
        push_task_diff_operations(task.id, &updated, Some(&task), &mut operations);

        if operations.is_empty() {
            return Ok(updated);
        }

        let actor_id = self.actor_id;
        let sequence = self.version_vector.next_sequence(actor_id)?;
        let timestamp = self.clock.timestamp();
        let change = Change {
            actor_id,
            sequence,
            timestamp,
            operations,
        };
        self.apply_change(&change)?;
        Ok(updated)
    }

    #[allow(clippy::too_many_lines)]
    pub fn apply_change(&mut self, change: &Change) -> Result<()> {
        log::trace!("{change:#?}");
        self.clock.merge(change.timestamp)?;
        let sequence = self.version_vector.apply(change)?;
        self.transaction
            .prepare_cached(indoc! {"
                UPDATE OR REPLACE actor_table SET
                    sequence = ?2,
                    timestamp_logical = ?3,
                    timestamp_counter = ?4
                WHERE
                    id = ?1
            "})?
            .execute((
                change.actor_id.get(),
                sequence.get(),
                change.timestamp.logical.get(),
                change.timestamp.counter,
            ))?;

        self.transaction
            .prepare_cached(indoc! {"
                INSERT INTO change_table (
                    actor_id,
                    sequence,
                    timestamp_logical,
                    timestamp_counter
                ) VALUES (?1, ?2, ?3, ?4)
            "})?
            .execute((
                change.actor_id.get(),
                change.sequence.get(),
                change.timestamp.logical.get(),
                change.timestamp.counter,
            ))?;

        let change_id = self.transaction.last_insert_rowid();

        for operation in &change.operations {
            let row_id = &operation.row_id;

            let operation_type = operation_type(&operation.kind);

            let overriding_operation_id = self
                .transaction
                .prepare_cached(indoc! {"
                    SELECT
                        ot.id
                    FROM
                        operation_table ot
                    LEFT JOIN
                        change_table ct ON ct.id = ot.change_id
                    WHERE
                        1=1
                        AND ot.row_id = ?1
                        AND ot.type = ?2
                        AND (ct.timestamp_logical > ?3 OR (ct.timestamp_logical = ?3 AND ct.timestamp_counter > ?4))
                    ORDER BY ct.timestamp_logical, ct.timestamp_counter DESC
                    LIMIT 1
                "})?
                .query_one((Sql::from(row_id), operation_type, change.timestamp.logical.get(), change.timestamp.counter), |row| row.get::<_, u64>("id"))
                .optional()?;

            if overriding_operation_id.is_none() {
                match &operation.kind {
                    OperationKind::Task(TaskOperation::Delete) => {
                        self.transaction
                            .prepare_cached(update_sql!("task_table", "tombstone"))?
                            .execute((Sql::from(row_id), 1))?;
                    }
                    OperationKind::Task(TaskOperation::ModifyEntry { entry }) => {
                        self.transaction
                            .prepare_cached(update_sql!("task_table", "entry"))?
                            .execute((Sql::from(row_id), Sql::from(*entry)))?;
                    }
                    OperationKind::Task(TaskOperation::ModifyTitle { title }) => {
                        self.transaction
                            .prepare_cached(update_sql!("task_table", "title"))?
                            .execute((Sql::from(row_id), title))?;
                    }
                    OperationKind::Task(TaskOperation::ModifyStatus { status }) => {
                        self.transaction
                            .prepare_cached(update_sql!("task_table", "status"))?
                            .execute((&Sql::from(row_id), &task_status_to_sql(*status)))?;
                    }
                    OperationKind::Task(TaskOperation::ModifyPriority { priority }) => {
                        self.transaction
                            .prepare_cached(update_sql!("task_table", "priority"))?
                            .execute((Sql::from(row_id), &priority.map(task_priority_to_sql)))?;
                    }
                    OperationKind::Task(TaskOperation::ModifyProject { project }) => {
                        self.transaction
                            .prepare_cached(update_sql!("task_table", "project"))?
                            .execute((Sql::from(row_id), project))?;
                    }
                    OperationKind::Task(TaskOperation::ModifyModified { modified }) => {
                        self.transaction
                            .prepare_cached(update_sql!("task_table", "modified"))?
                            .execute((Sql::from(row_id), &Sql::from(*modified)))?;
                    }
                    OperationKind::Task(TaskOperation::ModifyDue { due }) => {
                        self.transaction
                            .prepare_cached(update_sql!("task_table", "due"))?
                            .execute((Sql::from(row_id), &Sql::from(*due)))?;
                    }
                    OperationKind::Task(TaskOperation::ModifyWait { wait }) => {
                        self.transaction
                            .prepare_cached(update_sql!("task_table", "wait"))?
                            .execute((Sql::from(row_id), &Sql::from(*wait)))?;
                    }
                    OperationKind::Task(TaskOperation::AddTag { tag }) => {
                        self.transaction
                            .prepare_cached(
                                "INSERT INTO task_tag_table (task_id, tag_id) VALUES (?1, ?2) ON CONFLICT DO NOTHING",
                            )?
                            .execute((Sql::from(row_id), tag))?;
                    }
                    OperationKind::Task(TaskOperation::RemoveTag { tag }) => {
                        self.transaction
                            .prepare_cached(
                                "DELETE FROM task_tag_table WHERE task_id = ?1 AND tag_id = ?2",
                            )?
                            .execute((Sql::from(row_id), tag))?;
                    }
                    OperationKind::Task(TaskOperation::AddDependency { depend }) => {
                        self.transaction
                        .prepare_cached(
                            "INSERT INTO task_dependency_table (parent_task_id, child_task_id) VALUES (?1, ?2) ON CONFLICT DO NOTHING",
                        )?
                        .execute((Sql::from(row_id), depend))?;
                    }
                    OperationKind::Task(TaskOperation::RemoveDependency { depend }) => {
                        self.transaction
                        .prepare_cached(
                            "DELETE FROM task_dependency_table WHERE parent_task_id = ?1 AND child_task_id = ?2",
                        )?
                        .execute((Sql::from(row_id), depend))?;
                    }
                    OperationKind::Annotation(operation) => match operation {
                        AnnotationOperation::Delete => {
                            self.transaction
                                .prepare_cached(update_sql!("annotation_table", "tombstone"))?
                                .execute((Sql::from(row_id), 1))?;
                        }
                        AnnotationOperation::ModifyTaskId { task_id } => {
                            self.transaction
                                .prepare_cached(update_sql!("annotation_table", "task_id"))?
                                .execute((Sql::from(row_id), &task_id))?;
                        }
                        AnnotationOperation::ModifyEntry { entry } => {
                            self.transaction
                                .prepare_cached(update_sql!("annotation_table", "entry"))?
                                .execute((Sql::from(row_id), Sql::from(*entry)))?;
                        }
                        AnnotationOperation::ModifyText { text } => {
                            self.transaction
                                .prepare_cached(update_sql!("annotation_table", "text"))?
                                .execute((Sql::from(row_id), text))?;
                        }
                    },
                }
            }

            let mut operation_data = Vec::new();
            operation_to_data(&operation.kind, &mut operation_data);

            self.transaction
                .prepare_cached(indoc! {"
                    INSERT INTO operation_table (change_id, row_id, type, data)
                    VALUES (?1, ?2, ?3, ?4)
                "})?
                .execute((change_id, Sql::from(row_id), operation_type, operation_data))?;
        }
        Ok(())
    }

    fn row_to_operation(row: &rusqlite::Row<'_>) -> rusqlite::Result<Operation> {
        let row_id = row.get::<_, Uuid>("row_id")?;
        let typ = row.get::<_, u32>("type")?;
        let mut data = row.get_ref("data")?.as_blob()?;

        let operation = operation_from_data(typ, &mut data).unwrap();
        Ok(Operation {
            row_id: row_id.into(),
            kind: operation,
        })
    }

    pub fn changes(&mut self, actor_id: ActorId, change_range: ChangeRange) -> Result<Vec<Change>> {
        let mut change_sql = self.transaction.prepare(indoc! {"
            SELECT
                id,
                sequence,
                timestamp_logical,
                timestamp_counter
            FROM change_table
            WHERE
                actor_id = ?1 AND sequence > ?2
            ORDER BY sequence ASC
        "})?;

        let rows = change_sql.query_map((actor_id.get(), change_range.from), |row| {
            let id = row.get::<_, u64>("id")?;
            let sequence = row.get::<_, u64>("sequence")?;
            let timestamp_logical = row.get::<_, i64>("timestamp_logical")?;
            let timestamp_counter = row.get::<_, u32>("timestamp_counter")?;

            Ok((
                id,
                Change {
                    actor_id,
                    sequence: Sequence::new(sequence),
                    timestamp: Timestamp::new(
                        Microsecond::new(timestamp_logical),
                        timestamp_counter,
                    ),
                    operations: Vec::new(),
                },
            ))
        })?;

        let mut ids = Vec::new();
        let mut changes = Vec::new();
        for row in rows {
            let (id, change) = row?;
            ids.push(id);
            changes.push(change);
        }

        let mut operation_sql = self.transaction.prepare(indoc! {"
            SELECT
                `row_id`,
                `type`,
                `data`
            FROM operation_table
            WHERE
                change_id = ?1
            ORDER BY id ASC
        "})?;

        for (change, id) in changes.iter_mut().zip(ids) {
            let rows = operation_sql.query_map((id,), Self::row_to_operation)?;

            for row in rows {
                change.operations.push(row?);
            }
        }

        Ok(changes)
    }

    pub fn commit(self) -> Result<()> {
        let mut sql = self.transaction.prepare_cached(indoc! {"
            UPDATE actor_table SET
                sequence = ?2,
                timestamp_logical = ?3,
                timestamp_counter = ?4
            WHERE
                id = ?1
        "})?;

        for actor in self.version_vector.iter() {
            sql.execute((
                actor.id.get(),
                actor.sequence.get(),
                actor.timestamp.logical.get(),
                actor.timestamp.counter,
            ))?;
        }
        drop(sql);

        self.transaction.commit()?;
        Ok(())
    }
}
