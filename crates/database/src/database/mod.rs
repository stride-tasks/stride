//! Stride's sqlite database wrapper library.

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

mod functions;

use functions::init_stride_functions;
use rusqlite::{Connection, OptionalExtension, Row, ToSql, Transaction, types::Value};
use stride_core::{
    event::TaskQuery,
    task::{Annotation, Date, Task, TaskPriority, TaskStatus, Uda},
};
use uuid::Uuid;

use std::{collections::HashSet, path::Path, rc::Rc};

use crate::{
    Result, Sql, apply_migrations,
    conversion::{FromBlob, ToBlob, task_priority_to_sql},
    operation::{Operation, OperationKind},
    task_status_to_sql,
};

const SQL_ALL: &str = r"
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
    task.annotations,
    task.udas,
    string_agg(task_tag.tag_id, char(0)) AS tags
FROM
    task_table task
LEFT JOIN
    task_tag_table task_tag ON task_tag.task_id = task.id
WHERE
    task.status IN rarray(?1)
GROUP BY
    task.id
";

const SQL_BY_ID: &str = r"
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
    task.annotations,
    task.udas,
    string_agg(task_tag.tag_id, char(0)) AS tags
FROM
    task_table task
LEFT JOIN
    task_tag_table task_tag ON task_tag.task_id = task.id
WHERE
    task.id = ?1
GROUP BY
    task.id
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

#[allow(clippy::too_many_lines)]
fn push_operations_diff_task_common(current: &Task, previous: &Task, ops: &mut Vec<Operation>) {
    if current.status != previous.status {
        ops.push(
            OperationKind::TaskModifyStatus {
                id: current.uuid,
                new: current.status,
                old: previous.status,
            }
            .with_now(),
        );
    }
    if current.active != previous.active {
        ops.push(
            OperationKind::TaskModifyActive {
                id: current.uuid,
                new: current.active,
                old: previous.active,
            }
            .with_now(),
        );
    }
    if current.modified != previous.modified {
        ops.push(
            OperationKind::TaskModifyModified {
                id: current.uuid,
                new: current.modified,
                old: previous.modified,
            }
            .with_now(),
        );
    }
    if current.due != previous.due {
        ops.push(
            OperationKind::TaskModifyDue {
                id: current.uuid,
                new: current.due,
                old: previous.due,
            }
            .with_now(),
        );
    }
    if current.wait != previous.wait {
        ops.push(
            OperationKind::TaskModifyWait {
                id: current.uuid,
                new: current.wait,
                old: previous.wait,
            }
            .with_now(),
        );
    }
    if current.project != previous.project {
        ops.push(
            OperationKind::TaskModifyProject {
                id: current.uuid,
                new: current.project.clone().map(String::into_boxed_str),
                old: previous.project.clone().map(String::into_boxed_str),
            }
            .with_now(),
        );
    }
    if current.priority != previous.priority {
        ops.push(
            OperationKind::TaskModifyPriority {
                id: current.uuid,
                new: current.priority,
                old: previous.priority,
            }
            .with_now(),
        );
    }
    let current_tags = current
        .tags
        .iter()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    let previous_tags = previous
        .tags
        .iter()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    if current_tags != previous_tags {
        for tag in previous_tags.difference(&current_tags) {
            ops.push(
                OperationKind::TaskModifyRemoveTag {
                    id: current.uuid,
                    tag: (*tag).into(),
                }
                .with_now(),
            );
        }
        for tag in current_tags.difference(&previous_tags) {
            ops.push(
                OperationKind::TaskModifyAddTag {
                    id: current.uuid,
                    tag: (*tag).into(),
                }
                .with_now(),
            );
        }
    }
    let current_annotations = current.annotations.iter().collect::<HashSet<_>>();
    let previous_annotations = previous.annotations.iter().collect::<HashSet<_>>();
    if current_annotations != previous_annotations {
        for annotation in previous_annotations.difference(&current_annotations) {
            ops.push(
                OperationKind::TaskModifyRemoveAnnotation {
                    id: current.uuid,
                    annotation: Box::new((*annotation).clone()),
                }
                .with_now(),
            );
        }
        for annotation in current_annotations.difference(&previous_annotations) {
            ops.push(
                OperationKind::TaskModifyAddAnnotation {
                    id: current.uuid,
                    annotation: Box::new((*annotation).clone()),
                }
                .with_now(),
            );
        }
    }
}

fn push_operations_diff_task(current: &Task, previous: &Task, ops: &mut Vec<Operation>) {
    if current.title != previous.title {
        ops.push(
            OperationKind::TaskModifyTitle {
                id: current.uuid,
                new: current.title.clone().into_boxed_str(),
                old: previous.title.clone().into_boxed_str(),
            }
            .with_now(),
        );
    }
    if current.entry != previous.entry {
        ops.push(
            OperationKind::TaskModifyEntry {
                id: current.uuid,
                new: current.entry,
                old: previous.entry,
            }
            .with_now(),
        );
    }
    push_operations_diff_task_common(current, previous, ops);
}

fn push_operations_diff_task_with_default(current: &Task, ops: &mut Vec<Operation>) {
    ops.push(
        OperationKind::TaskCreate {
            id: current.uuid,
            title: current.title.clone().into(),
        }
        .with_now(),
    );

    let previous = Task::default();
    push_operations_diff_task_common(current, &previous, ops);

    // task.depends;
    // task.udas;
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
            depends: Vec::new(),
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

    pub fn get_undoable_operation(&self, mut limit: usize) -> Result<Vec<Operation>> {
        let mut operations = Vec::new();

        let mut sql =
            self.prepare("SELECT timestamp, kind FROM operation_table ORDER BY id DESC")?;
        let operations_rows = sql.query_map((), |row| {
            let timestamp = row.get::<_, Sql<Date>>("timestamp")?;
            let kind = row.get::<_, Option<Vec<u8>>>("kind")?;

            Ok(kind
                .as_deref()
                .map(|mut blob| OperationKind::from_blob(&mut blob))
                .transpose()
                .map(|kind| Operation {
                    timestamp: timestamp.value,
                    kind,
                }))
        })?;
        for operation in operations_rows {
            let operation = operation??;
            let is_undo_point = operation.is_undo_point();
            operations.push(operation);
            if is_undo_point {
                limit = limit.saturating_sub(1);
                if limit == 0 {
                    break;
                }
            }
        }
        Ok(operations)
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

    #[allow(clippy::too_many_lines)]
    pub fn undo(&mut self, mut undo_count: usize) -> Result<()> {
        let transaction = self.transaction()?;

        let mut operations = Vec::new();

        let mut sql = transaction
            .prepare("SELECT id, timestamp, kind FROM operation_table ORDER BY id DESC")?;
        let operations_rows = sql.query_map((), |row| {
            let id = row.get::<_, i64>("id")?;
            let timestamp = row.get::<_, Sql<Date>>("timestamp")?;
            let kind = row.get::<_, Option<Vec<u8>>>("kind")?;

            Ok(kind
                .as_deref()
                .map(|mut blob| OperationKind::from_blob(&mut blob))
                .transpose()
                .map(|kind| {
                    (
                        id,
                        Operation {
                            timestamp: timestamp.value,
                            kind,
                        },
                    )
                }))
        })?;
        for operation in operations_rows {
            let (id, operation) = operation??;
            let is_undo_point = operation.is_undo_point();
            operations.push((id, operation));
            if is_undo_point {
                undo_count = undo_count.saturating_sub(1);
                if undo_count == 0 {
                    break;
                }
            }
        }

        drop(sql);

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
            }
        }

        transaction.commit()?;
        Ok(())
    }
}
