//! Stride's sqlite database wrapper library.

#![allow(clippy::missing_errors_doc)]

pub mod conversion;
mod error;
mod migrations;

use conversion::{Sql, task_status_to_sql};
pub use error::{Error, Result};

use migrations::apply_migrations;
use rusqlite::{Connection, OptionalExtension, Row, ToSql, types::Value};
use stride_core::{
    event::TaskQuery,
    task::{Annotation, Date, Task, TaskPriority, TaskStatus, Uda},
};
use uuid::Uuid;

use std::{collections::HashSet, path::Path, rc::Rc};

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

    pub fn insert_task(&mut self, task: &Task) -> Result<()> {
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
        transaction.commit()?;
        Ok(())
    }

    pub fn update_task(&mut self, task: &Task) -> Result<()> {
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
}
