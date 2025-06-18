mod functions;

use functions::init_stride_functions;
use rusqlite::{Connection, OptionalExtension, Row, ToSql, Transaction, types::Value};
use stride_core::{
    backend::BackendRecord,
    event::TaskQuery,
    task::{Annotation, Date, Task, TaskPriority, TaskStatus, Uda},
};
use uuid::Uuid;

use std::{collections::HashSet, path::Path, rc::Rc};

use crate::{
    Result, Sql, apply_migrations,
    conversion::{FromBlob, ToBlob, task_priority_to_sql},
    operation::{
        Operation, OperationKind,
        difference::{push_operations_diff_task, push_operations_diff_task_with_default},
    },
    task_status_to_sql,
};

const SQL_ALL: &str = r"
WITH task_depends_cte AS (
    SELECT
        parent_task_id AS id,
        GROUP_CONCAT(child_task_id) AS depends
    FROM
        task_dependency_table
    GROUP BY
        parent_task_id
),
task_tags_cte AS (
    SELECT
        task_id,
        string_agg(tag_id, char(0)) AS tags
    FROM
        task_tag_table
    GROUP BY
        task_id
)

SELECT
    task.id,
    task.title,
    task.entry,
    task.status,
    task.priority,
    task.project,
    task.modified,
    task.due,
    task.wait,
    depends_cte.depends,
    task.annotations,
    task.udas,
    tags_cte.tags
FROM
    task_table task
LEFT JOIN
    task_depends_cte depends_cte ON depends_cte.id = task.id
LEFT JOIN
    task_tags_cte tags_cte ON tags_cte.task_id = task.id
WHERE
    task.status IN rarray(?1)
";

const SQL_BY_ID: &str = r"
WITH task_depends_cte AS (
    SELECT
        parent_task_id AS id,
        GROUP_CONCAT(child_task_id) AS depends
    FROM
        task_dependency_table
    GROUP BY
        parent_task_id
),
task_tags_cte AS (
    SELECT
        task_id,
        string_agg(tag_id, char(0)) AS tags
    FROM
        task_tag_table
    GROUP BY
        task_id
)

SELECT
    task.id,
    task.title,
    task.entry,
    task.status,
    task.priority,
    task.project,
    task.modified,
    task.due,
    task.wait,
    depends_cte.depends,
    task.annotations,
    task.udas,
    tags_cte.tags
FROM
    task_table task
LEFT JOIN
    task_depends_cte depends_cte ON depends_cte.id = task.id
LEFT JOIN
    task_tags_cte tags_cte ON tags_cte.task_id = task.id
WHERE
    task.id = ?1
";

const SQL_INSERT: &str = r"
INSERT INTO task_table (
    id,
    title,
    entry,
    status,
    priority,
    project,
    modified,
    due,
    wait,
    annotations,
    udas
)
VALUES
(
    :id,
    :title,
    :entry,
    :status,
    :priority,
    :project,
    :modified,
    :due,
    :wait,
    :annotations,
    :udas
);
";

const SQL_UPDATE: &str = r"
UPDATE task_table
SET
    title = :title,
    entry = :entry,
    status = :status,
    priority = :priority,
    project = :project,
    modified = :modified,
    due = :due,
    wait = :wait,
    annotations = :annotations,
    udas = :udas
WHERE id = :id;
";

const SQL_DELETE: &str = r"DELETE FROM task_table WHERE id = ?1";

const SQL_PROJECT_INSERT_OR_IGNORE: &str = "INSERT OR IGNORE INTO project_table (id) VALUES (?1);";
const SQL_TAG_INSERT_OR_IGNORE: &str = "INSERT OR IGNORE INTO tag_table (id) VALUES (?1);";

#[derive(Debug)]
pub struct TaskTransaction {
    task: Task,
}

impl TaskTransaction {
    pub fn create(id: Uuid, title: String, ops: &mut Vec<Operation>) -> Self {
        ops.push(
            OperationKind::TaskCreate {
                id,
                title: title.clone().into_boxed_str(),
            }
            .with_now(),
        );
        Self {
            task: Task::with_uuid(id, title),
        }
    }

    pub fn set_title(&mut self, title: String, ops: &mut Vec<Operation>) {
        let new = title.clone();
        let old = std::mem::replace(&mut self.task.title, title);
        ops.push(
            OperationKind::TaskModifyTitle {
                id: self.task.uuid,
                new: new.into_boxed_str(),
                old: old.into_boxed_str(),
            }
            .with_now(),
        );
    }

    pub fn set_status(&mut self, status: TaskStatus, ops: &mut Vec<Operation>) {
        let new = status;
        let old = std::mem::replace(&mut self.task.status, status);
        ops.push(
            OperationKind::TaskModifyStatus {
                id: self.task.uuid,
                new,
                old,
            }
            .with_now(),
        );
    }
    pub fn set_entry(&mut self, entry: Date, ops: &mut Vec<Operation>) {
        let new = entry;
        let old = std::mem::replace(&mut self.task.entry, entry);
        ops.push(
            OperationKind::TaskModifyEntry {
                id: self.task.uuid,
                new,
                old,
            }
            .with_now(),
        );
    }
    pub fn set_modified(&mut self, modified: Option<Date>, ops: &mut Vec<Operation>) {
        let new = modified;
        let old = std::mem::replace(&mut self.task.modified, modified);
        ops.push(
            OperationKind::TaskModifyModified {
                id: self.task.uuid,
                new,
                old,
            }
            .with_now(),
        );
    }
    pub fn set_due(&mut self, due: Option<Date>, ops: &mut Vec<Operation>) {
        let new = due;
        let old = std::mem::replace(&mut self.task.due, due);
        ops.push(
            OperationKind::TaskModifyDue {
                id: self.task.uuid,
                new,
                old,
            }
            .with_now(),
        );
    }
    pub fn set_project(&mut self, project: Option<String>, ops: &mut Vec<Operation>) {
        let new = project.clone();
        let old = std::mem::replace(&mut self.task.project, project);
        ops.push(
            OperationKind::TaskModifyProject {
                id: self.task.uuid,
                new: new.map(String::into_boxed_str),
                old: old.map(String::into_boxed_str),
            }
            .with_now(),
        );
    }
    pub fn set_priority(&mut self, priority: Option<TaskPriority>, ops: &mut Vec<Operation>) {
        let new = priority;
        let old = std::mem::replace(&mut self.task.priority, priority);
        ops.push(
            OperationKind::TaskModifyPriority {
                id: self.task.uuid,
                new,
                old,
            }
            .with_now(),
        );
    }
    pub fn set_wait(&mut self, wait: Option<Date>, ops: &mut Vec<Operation>) {
        let new = wait;
        let old = std::mem::replace(&mut self.task.wait, wait);
        ops.push(
            OperationKind::TaskModifyWait {
                id: self.task.uuid,
                new,
                old,
            }
            .with_now(),
        );
    }
}

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl std::ops::Deref for Database {
    type Target = Connection;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl std::ops::DerefMut for Database {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}

impl Database {
    #[inline]
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        rusqlite::vtab::array::load_module(&conn)?;
        init_stride_functions(&conn)?;
        Ok(Self { conn })
    }

    #[inline]
    pub fn apply_migrations(&mut self) -> Result<()> {
        apply_migrations(self)?;
        Ok(())
    }

    fn row_to_task(row: &Row<'_>) -> Result<Task, rusqlite::Error> {
        let uuid = row.get::<_, Uuid>("id")?;
        let title = row.get::<_, String>("title")?;
        let entry = row.get::<_, Sql<Date>>("entry")?.value;
        let mut status = row.get::<_, Sql<TaskStatus>>("status")?.value;
        let priority = row.get::<_, Sql<Option<TaskPriority>>>("priority")?.value;
        let project = row.get::<_, Option<String>>("project")?;
        let modified = row.get::<_, Sql<Option<Date>>>("modified")?.value;
        let due = row.get::<_, Sql<Option<Date>>>("due")?.value;
        let wait = row.get::<_, Sql<Option<Date>>>("wait")?.value;
        let depends = row.get::<_, Sql<Vec<Uuid>>>("depends")?.value;
        let annotations = row.get::<_, Sql<Vec<Annotation>>>("annotations")?.value;
        let udas = row.get::<_, Sql<Vec<Uda>>>("udas")?.value;
        let tags = row
            .get::<_, Option<String>>("tags")?
            .map(|tags| tags.split('\0').map(String::from).collect::<Vec<_>>())
            .unwrap_or_default();

        if wait.is_some() {
            status = TaskStatus::Waiting;
        }

        Ok(Task {
            uuid,
            entry,
            status,
            title,
            active: false,
            modified,
            due,
            project,
            tags,
            annotations,
            priority,
            wait,
            depends,
            udas,
        })
    }

    pub fn all_tasks(&mut self) -> Result<Vec<Task>> {
        self.tasks_by_status(
            &[
                TaskStatus::Pending,
                TaskStatus::Complete,
                TaskStatus::Deleted,
                TaskStatus::Waiting,
                TaskStatus::Recurring,
            ]
            .into(),
        )
    }

    pub fn task_query(&mut self, query: &TaskQuery) -> Result<Vec<Task>> {
        match query {
            TaskQuery::Uuid { uuid } => {
                let Some(tasks) = self.task_by_id(*uuid).transpose() else {
                    return Ok(Vec::new());
                };
                Ok(vec![tasks?])
            }
            TaskQuery::Title {
                title,
                status,
                limit,
            } => {
                // TODO: Optimize this by using sql directly.
                let title = title.to_lowercase();
                let mut tasks = self.tasks_by_status(status)?;
                tasks.retain(|task| task.title.to_lowercase() == title);
                tasks.truncate(limit.unwrap_or(u32::MAX) as usize);
                tasks.sort_unstable_by(|a, b| b.urgency().total_cmp(&a.urgency()));
                Ok(tasks)
            }
        }
    }

    pub fn tasks_by_status(&mut self, status: &HashSet<TaskStatus>) -> Result<Vec<Task>> {
        let mut tasks = Vec::new();
        let mut sql = self.prepare_cached(SQL_ALL)?;
        let statys_array = Rc::new(
            status
                .iter()
                .copied()
                .map(task_status_to_sql)
                .map(Value::from)
                .collect::<Vec<_>>(),
        );
        let task_iter = sql.query_map([statys_array], Self::row_to_task)?;
        for task in task_iter {
            let task = task?;
            if status.contains(&task.status) {
                tasks.push(task);
            }
        }
        tasks.sort_unstable_by(|a, b| b.urgency().total_cmp(&a.urgency()));
        Ok(tasks)
    }

    pub fn tasks_by_title(&mut self, title: &str) -> Result<Vec<Task>> {
        let title = title.to_lowercase();
        let mut tasks = Vec::new();
        let mut sql = self.prepare_cached(SQL_ALL)?;
        let task_iter = sql.query_map([], Self::row_to_task)?;
        for task in task_iter {
            let task = task?;
            if task.title.to_lowercase() == title {
                tasks.push(task);
            }
        }
        tasks.sort_unstable_by(|a, b| b.urgency().total_cmp(&a.urgency()));
        Ok(tasks)
    }

    pub fn task_by_id(&mut self, id: Uuid) -> Result<Option<Task>> {
        self.query_row(SQL_BY_ID, (id,), Self::row_to_task)
            .optional()
            .map_err(Into::into)
    }

    pub fn task_create(
        &mut self,
        id: Uuid,
        title: String,
        ops: &mut Vec<Operation>,
    ) -> Result<TaskTransaction> {
        Ok(TaskTransaction::create(id, title, ops))
    }

    pub fn insert_task(&mut self, task: &Task) -> Result<()> {
        let mut operations = Vec::new();

        operations.push(Operation::undo_point_with_now());
        push_operations_diff_task_with_default(task, &mut operations);

        let transaction = self.transaction()?;

        if let Some(project) = &task.project {
            let mut sql = transaction.prepare_cached(SQL_PROJECT_INSERT_OR_IGNORE)?;
            sql.execute((project,))?;
        }

        let mut sql = transaction.prepare_cached(SQL_INSERT)?;

        let mut task_uuid = task.uuid;
        if task.uuid == Uuid::nil() {
            task_uuid = Uuid::now_v7();
        }

        sql.execute::<&[(&str, &dyn ToSql)]>(&[
            (":id", &task_uuid),
            (":title", &task.title),
            (":entry", &Sql::from(task.entry)),
            (":status", &Sql::from(task.status)),
            (":priority", &Sql::from(task.priority)),
            (":project", &task.project),
            (":modified", &Sql::from(task.modified)),
            (":due", &Sql::from(task.due)),
            (":wait", &Sql::from(task.wait)),
            (":annotations", &Sql::from(task.annotations.as_slice())),
            (":udas", &Sql::from(task.udas.as_slice())),
        ])?;

        if !task.tags.is_empty() {
            let mut sql = transaction.prepare_cached(SQL_TAG_INSERT_OR_IGNORE)?;
            for tag in &task.tags {
                sql.execute((tag,))?;
            }

            let mut sql = transaction
                .prepare_cached("INSERT INTO task_tag_table (task_id, tag_id) VALUES (?1, ?2) ON CONFLICT DO NOTHING")?;
            for tag in &task.tags {
                sql.execute((task_uuid, tag))?;
            }
        }
        drop(sql);
        let mut sql = transaction
            .prepare_cached("INSERT INTO operation_table (timestamp, kind) VALUES (?1, ?2)")?;

        for operation in operations {
            sql.execute((
                &Sql::from(operation.timestamp),
                &operation.kind.as_ref().map(|kind| {
                    let mut blob = Vec::new();
                    kind.to_blob(&mut blob);
                    blob
                }),
            ))?;
        }

        drop(sql);
        transaction.commit()?;
        Ok(())
    }

    pub fn update_task(&mut self, task: &Task) -> Result<()> {
        let Some(previous) = self.task_by_id(task.uuid)? else {
            return self.insert_task(task);
        };

        let mut operations = Vec::new();
        push_operations_diff_task(task, &previous, &mut operations);
        if !operations.is_empty() {
            operations.insert(0, Operation::undo_point_with_now());
        }

        let transaction = self.transaction()?;

        if let Some(project) = &task.project {
            let mut sql = transaction.prepare_cached(SQL_PROJECT_INSERT_OR_IGNORE)?;
            sql.execute((project,))?;
        }

        let mut sql = transaction.prepare_cached(SQL_UPDATE)?;
        sql.execute::<&[(&str, &dyn ToSql)]>(&[
            (":id", &task.uuid),
            (":title", &task.title),
            (":entry", &Sql::from(task.entry)),
            (":status", &Sql::from(task.status)),
            (":priority", &Sql::from(task.priority)),
            (":project", &task.project),
            (":modified", &Sql::from(task.modified)),
            (":due", &Sql::from(task.due)),
            (":wait", &Sql::from(task.wait)),
            (":annotations", &Sql::from(task.annotations.as_slice())),
            (":udas", &Sql::from(task.udas.as_slice())),
        ])?;
        drop(sql);

        if !task.tags.is_empty() {
            let mut sql = transaction.prepare_cached(SQL_TAG_INSERT_OR_IGNORE)?;
            for tag in &task.tags {
                sql.execute((tag,))?;
            }

            // TODO: Maybe instead of deleting them all,
            // figure out a nice way to delete only the tags that are old.
            transaction.execute(
                "DELETE FROM task_tag_table WHERE task_id = ?1",
                (task.uuid,),
            )?;
            let mut sql = transaction
                .prepare_cached("INSERT INTO task_tag_table (task_id, tag_id) VALUES (?1, ?2) ON CONFLICT DO NOTHING")?;
            for tag in &task.tags {
                sql.execute((task.uuid, tag))?;
            }
        }

        let mut sql = transaction
            .prepare_cached("INSERT INTO operation_table (timestamp, kind) VALUES (?1, ?2)")?;

        for operation in operations {
            sql.execute((
                &Sql::from(operation.timestamp),
                &operation.kind.as_ref().map(|kind| {
                    let mut blob = Vec::new();
                    kind.to_blob(&mut blob);
                    blob
                }),
            ))?;
        }

        drop(sql);
        transaction.commit()?;
        Ok(())
    }

    pub fn purge_task_by_id(&mut self, id: Uuid) -> Result<Option<Task>> {
        let task = self.task_by_id(id)?;
        if task.is_some() {
            self.execute(SQL_DELETE, (id,))?;
        }
        Ok(task)
    }

    pub fn update_task_modified_property(
        transaction: &Transaction<'_>,
        id: Uuid,
        timestamp: Option<Date>,
    ) -> Result<()> {
        transaction.execute(
            "UPDATE task_table SET modified = ?2 WHERE id = ?1",
            (id, &Sql::from(timestamp)),
        )?;
        Ok(())
    }

    pub fn undoable_operation(&mut self, undo_count: usize) -> Result<Vec<(i64, Operation)>> {
        let transaction = self.transaction()?;
        Self::get_undoable_operation(&transaction, undo_count)
    }

    pub fn get_undoable_operation(
        transaction: &Transaction<'_>,
        mut limit: usize,
    ) -> Result<Vec<(i64, Operation)>> {
        let mut operations = Vec::new();

        let mut sql = transaction
            .prepare("SELECT id, timestamp, kind FROM operation_table ORDER BY id DESC")?;
        let operations_rows = sql.query_map((), |row| {
            let id = row.get::<_, i64>("id")?;
            let timestamp = row.get::<_, Sql<Date>>("timestamp")?;
            let kind = row.get::<_, Option<Vec<u8>>>("kind")?;

            Ok((
                id,
                kind.as_deref()
                    .map(|mut blob| OperationKind::from_blob(&mut blob))
                    .transpose()
                    .map(|kind| Operation {
                        timestamp: timestamp.value,
                        kind,
                    })
                    .map_err(|e| rusqlite::types::FromSqlError::Other(e.into()))?,
            ))
        })?;
        for operation in operations_rows {
            let (id, operation) = operation?;
            let is_undo_point = operation.is_undo_point();
            operations.push((id, operation));
            if is_undo_point {
                limit = limit.saturating_sub(1);
                if limit == 0 {
                    break;
                }
            }
        }
        Ok(operations)
    }

    #[allow(clippy::too_many_lines)]
    pub fn undo(&mut self, undo_count: usize) -> Result<()> {
        let transaction = self.transaction()?;

        let operations = Self::get_undoable_operation(&transaction, undo_count)?;
        for (id, Operation { timestamp, kind }) in operations {
            transaction.execute("DELETE FROM operation_table WHERE id = ?1", (id,))?;
            let Some(kind) = kind else {
                continue;
            };
            match kind {
                OperationKind::TaskCreate { id, .. } => {
                    transaction.execute("DELETE FROM task_table WHERE id = ?1", (id,))?;
                }
                OperationKind::TaskPurge { .. } => todo!(),
                OperationKind::TaskModifyEntry { id, old, .. } => {
                    transaction.execute(
                        "UPDATE task_table SET entry = ?2 WHERE id = ?1",
                        (id, &Sql::from(old)),
                    )?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
                OperationKind::TaskModifyTitle { id, old, .. } => {
                    transaction
                        .execute("UPDATE task_table SET title = ?2 WHERE id = ?1", (id, &old))?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
                OperationKind::TaskModifyStatus { id, old, .. } => {
                    transaction.execute(
                        "UPDATE task_table SET status = ?2 WHERE id = ?1",
                        (id, &task_status_to_sql(old)),
                    )?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
                OperationKind::TaskModifyActive { .. } => todo!(),
                OperationKind::TaskModifyPriority { id, old, .. } => {
                    transaction.execute(
                        "UPDATE task_table SET priority = ?2 WHERE id = ?1",
                        (id, &old.map(task_priority_to_sql)),
                    )?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
                OperationKind::TaskModifyProject { id, old, .. } => {
                    transaction.execute(
                        "UPDATE task_table SET project = ?2 WHERE id = ?1",
                        (id, &old),
                    )?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
                OperationKind::TaskModifyModified { id, old, .. } => {
                    Self::update_task_modified_property(&transaction, id, old)?;
                }
                OperationKind::TaskModifyDue { id, old, .. } => {
                    transaction.execute(
                        "UPDATE task_table SET due = ?2 WHERE id = ?1",
                        (id, &Sql::from(old)),
                    )?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
                OperationKind::TaskModifyWait { id, new: _, old } => {
                    transaction.execute(
                        "UPDATE task_table SET wait = ?2 WHERE id = ?1",
                        (id, &Sql::from(old)),
                    )?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
                OperationKind::TaskModifyAddTag { id, tag } => {
                    transaction.execute(
                        "DELETE FROM task_tag_table WHERE task_id = ?1 AND tag_id = ?2",
                        (id, tag),
                    )?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
                OperationKind::TaskModifyRemoveTag { id, tag } => {
                    transaction.execute(
                        "INSERT INTO task_tag_table (task_id, tag_id) VALUES (?1, ?2)",
                        (id, tag),
                    )?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
                OperationKind::TaskModifyAddDependency { id, depend } => {
                    transaction.execute(
                        "DELETE FROM task_dependency_table WHERE parent_task_id = ?1 AND child_task_id = ?2",
                        (&id, &depend),
                    )?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
                OperationKind::TaskModifyRemoveDependency { id, depend } => {
                    transaction.execute(
                        "INSERT INTO task_dependency_table (parent_task_id, child_task_id) VALUES (?1, ?2) ON CONFLICT DO NOTHING",
                        (&id, &depend),
                    )?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
                OperationKind::TaskModifyAddAnnotation { id, annotation } => {
                    let mut annotation_blob = Vec::new();
                    annotation.to_blob(&mut annotation_blob);
                    transaction.execute(
                        "UPDATE task_table SET annotations = stride_annotation_array_remove(annotations, ?2) WHERE id = ?1",
                        (id, &annotation_blob),
                    )?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
                OperationKind::TaskModifyRemoveAnnotation { id, annotation } => {
                    let mut annotation_blob = Vec::new();
                    annotation.to_blob(&mut annotation_blob);
                    transaction.execute(
                        "UPDATE task_table SET annotations = stride_annotation_array_insert(annotations, ?2) WHERE id = ?1",
                        (id, &annotation_blob),
                    )?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
                OperationKind::TaskModifyAddUda { id, uda } => {
                    let mut uda_blob = Vec::new();
                    uda.to_blob(&mut uda_blob);
                    transaction.execute(
                        "UPDATE task_table SET udas = stride_uda_array_remove(udas, ?2) WHERE id = ?1",
                        (id, &uda_blob),
                    )?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
                OperationKind::TaskModifyRemoveUda { id, uda } => {
                    let mut uda_blob = Vec::new();
                    uda.to_blob(&mut uda_blob);
                    transaction.execute(
                        "UPDATE task_table SET udas = stride_uda_array_insert(udas, ?2) WHERE id = ?1",
                        (id, &uda_blob),
                    )?;
                    Self::update_task_modified_property(&transaction, id, Some(timestamp))?;
                }
            }
        }

        transaction.commit()?;
        Ok(())
    }

    pub fn add_backends(&mut self, backend: &BackendRecord) -> Result<()> {
        let property = serde_json::to_string(&backend.config).expect("should not fail");
        self.execute(
            "INSERT INTO backend_table (
    id,
    name,
    enabled,
    property)
VALUES (?1, ?2,
?3,
?4)",
            (
                backend.id.as_bytes(),
                &backend.name,
                &backend.enabled,
                &property,
            ),
        )?;
        Ok(())
    }

    pub fn backends(&mut self) -> Result<Vec<BackendRecord>> {
        let mut sql = self.prepare("SELECT id, name, enabled, property FROM backend_table")?;
        let rows = sql.query_map((), |row| {
            let config = row.get::<_, String>("property")?;
            let config = serde_json::from_str(&config)
                .map_err(|error| rusqlite::types::FromSqlError::Other(error.into()))?;
            Ok(BackendRecord {
                id: row.get::<_, Uuid>("id")?,
                name: row.get::<_, Box<str>>("name")?,
                enabled: row.get::<_, bool>("enabled")?,
                config,
            })
        })?;

        let mut backends = Vec::new();
        for row in rows {
            backends.push(row?);
        }
        Ok(backends)
    }

    pub fn update_backend(&mut self, backend: &BackendRecord) -> Result<()> {
        let config = serde_json::to_string(&backend.config).expect("should not fail");
        self.execute(
            "UPDATE backend_table
SET name = ?2,
    enabled = ?3,
    property = ?4
WHERE id = ?1",
            (
                backend.id.as_bytes(),
                &backend.name,
                &backend.enabled,
                &config,
            ),
        )?;
        Ok(())
    }

    pub fn delete_backend(&mut self, id: Uuid) -> Result<()> {
        self.execute("DELETE FROM backend_table WHERE id = ?1", (id.as_bytes(),))?;
        Ok(())
    }
}
