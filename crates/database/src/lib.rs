mod conversion;
mod error;
mod migration;

use conversion::Sql;
pub use error::{Error, Result};
use migration::MIGRATIONS;

use rusqlite::{Connection, ToSql};
use stride_core::task::{Date, Task, TaskPriority, TaskStatus};
use uuid::Uuid;

use std::{collections::HashMap, path::Path};

const SQL_ALL: &str = r"
SELECT
    id,
    title,
    entry,
    status,
    priority,
    project,
    modified,
    due,
    wait
FROM task_table task;
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
    wait
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
    :wait
);
";

const SQL_PROJECT_INSERT_OR_IGNORE: &str = r"
INSERT OR IGNORE INTO project_table (id)
VALUES (:project_name);
";

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
        Ok(Self { conn })
    }

    #[inline]
    pub fn apply_migrations(&mut self) -> Result<()> {
        MIGRATIONS.apply(&mut self.conn)
    }

    pub fn all_tasks(&mut self) -> Result<Vec<Task>> {
        let mut tasks = Vec::new();
        let mut sql = self.prepare_cached(SQL_ALL)?;
        let task_iter = sql.query_map([], |row| {
            let uuid = row.get::<_, Uuid>("id")?;
            let title = row.get::<_, String>("title")?;
            let _entry = row.get::<_, Sql<Date>>("entry")?.value;
            let mut status = row.get::<_, Sql<TaskStatus>>("status")?.value;
            let priority = row.get::<_, Sql<Option<TaskPriority>>>("priority")?.value;
            let project = row.get::<_, Option<String>>("project")?;
            let modified = row.get::<_, Sql<Option<Date>>>("modified")?.value;
            let due = row.get::<_, Sql<Option<Date>>>("due")?.value;
            let wait = row.get::<_, Sql<Option<Date>>>("wait")?.value;

            if wait.is_some() {
                status = TaskStatus::Waiting;
            }

            return Ok(Task {
                uuid,
                status,
                title,
                active: false,
                modified,
                due,
                project,
                tags: Vec::new(),
                annotations: Vec::new(),
                priority,
                wait,
                depends: Vec::new(),
                uda: HashMap::new(),
            });
        })?;
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    pub fn insert_task(&mut self, task: &Task) -> Result<()> {
        let transaction = self.transaction()?;

        if let Some(project) = &task.project {
            let mut sql = transaction.prepare_cached(SQL_PROJECT_INSERT_OR_IGNORE)?;
            sql.execute(&[(":project_name", project)])?;
        }

        let mut sql = transaction.prepare_cached(SQL_INSERT)?;

        let mut task_uuid = task.uuid;
        if task.uuid == Uuid::nil() {
            task_uuid = Uuid::now_v7();
        }
        sql.execute::<&[(&str, &dyn ToSql)]>(&[
            (":id", &task_uuid),
            (":title", &task.title),
            (":entry", &Sql::from(task.entry())),
            (":status", &Sql::from(task.status)),
            (":priority", &Sql::from(task.priority)),
            (":project", &task.project),
            (":modified", &Sql::from(task.modified)),
            (":due", &Sql::from(task.due)),
            (":wait", &Sql::from(task.wait)),
        ])?;

        drop(sql);
        transaction.commit()?;
        Ok(())
    }
}
