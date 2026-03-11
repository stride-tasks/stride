#[cfg(test)]
mod tests;

use chrono::DateTime;
use indoc::indoc;
use rusqlite::{Connection, OptionalExtension, Row, functions::FunctionFlags};
use stride_core::{
    backend::{BackendRecord, Config, Value},
    event::TaskQuery,
    task::{Annotation, Date, Task, TaskPriority, TaskStatus, Uda},
};
use stride_crdt::{actor::ActorId, hlc::Clock};
use url::Url;
use uuid::Uuid;

use std::{
    collections::{HashMap, HashSet},
    path::Path,
    rc::Rc,
};

use crate::{
    Result, Sql, apply_migrations, database::transaction::Transaction, task_status_to_sql,
};

pub(crate) mod transaction;

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
),
task_annotations_cte AS (
    SELECT
        task_id,
        stride_blob_concat(id) AS ids,
        string_agg(entry, ' ') AS entries,
        string_agg(text, char(0)) AS texts
    FROM
        annotation_table
    WHERE
        tombstone = 0
    GROUP BY
        task_id
),
task_udas_cte AS (
    SELECT
        task_id,
        string_agg(key, char(0)) AS keys,
        string_agg(value, char(0)) AS values_
    FROM
        uda_table
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
    annotations_cte.ids AS annotation_ids,
    annotations_cte.entries AS annotation_entries,
    annotations_cte.texts AS annotation_texts,
    udas_cte.keys AS uda_keys,
    udas_cte.values_ AS uda_values,
    tags_cte.tags
FROM
    task_table task
LEFT JOIN
    task_depends_cte depends_cte ON depends_cte.id = task.id
LEFT JOIN
    task_tags_cte tags_cte ON tags_cte.task_id = task.id
LEFT JOIN
    task_annotations_cte annotations_cte ON annotations_cte.task_id = task.id
LEFT JOIN
    task_udas_cte udas_cte ON udas_cte.task_id = task.id
WHERE
    task.tombstone = 0
    AND task.status IN rarray(?1)
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
),
task_annotations_cte AS (
    SELECT
        task_id,
        stride_blob_concat(id) AS ids,
        string_agg(entry, ' ') AS entries,
        string_agg(text, char(0)) AS texts
    FROM
        annotation_table
    WHERE
        tombstone = 0
    GROUP BY
        task_id
),
task_udas_cte AS (
    SELECT
        task_id,
        string_agg(key, char(0)) AS keys,
        string_agg(value, char(0)) AS values_
    FROM
        uda_table
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
    annotations_cte.ids AS annotation_ids,
    annotations_cte.entries AS annotation_entries,
    annotations_cte.texts AS annotation_texts,
    udas_cte.keys AS uda_keys,
    udas_cte.values_ AS uda_values,
    tags_cte.tags
FROM
    task_table task
LEFT JOIN
    task_depends_cte depends_cte ON depends_cte.id = task.id
LEFT JOIN
    task_tags_cte tags_cte ON tags_cte.task_id = task.id
LEFT JOIN
    task_annotations_cte annotations_cte ON annotations_cte.task_id = task.id
LEFT JOIN
    task_udas_cte udas_cte ON udas_cte.task_id = task.id
WHERE
    task.id = ?1
    AND task.tombstone = 0
";

#[derive(Debug)]
pub struct Database {
    actor_id: ActorId,
    pub clock: Clock,
    pub(crate) connection: Connection,
}

impl Database {
    #[inline]
    pub fn open(path: &Path, actor_id: ActorId, clock: Clock) -> Result<Self> {
        let connection = Connection::open(path)?;
        Self::init(connection, actor_id, clock)
    }

    #[inline]
    pub fn open_in_memory(actor_id: ActorId, clock: Clock) -> Result<Self> {
        let connection = Connection::open_in_memory()?;
        Self::init(connection, actor_id, clock)
    }

    #[inline]
    fn init(connection: Connection, actor_id: ActorId, clock: Clock) -> Result<Self> {
        struct BlobConcat;
        impl rusqlite::functions::Aggregate<Vec<u8>, Vec<u8>> for BlobConcat {
            fn init(&self, _: &mut rusqlite::functions::Context<'_>) -> rusqlite::Result<Vec<u8>> {
                Ok(Vec::new())
            }
            fn step(
                &self,
                ctx: &mut rusqlite::functions::Context<'_>,
                acc: &mut Vec<u8>,
            ) -> rusqlite::Result<()> {
                let arg = ctx.get::<Vec<u8>>(0)?;
                acc.extend_from_slice(&arg);
                Ok(())
            }
            fn finalize(
                &self,
                _: &mut rusqlite::functions::Context<'_>,
                acc: Option<Vec<u8>>,
            ) -> rusqlite::Result<Vec<u8>> {
                Ok(acc.unwrap_or_default())
            }
        }
        connection.create_aggregate_function(
            "stride_blob_concat",
            1,
            FunctionFlags::SQLITE_DETERMINISTIC,
            BlobConcat,
        )?;
        rusqlite::vtab::array::load_module(&connection)?;
        Ok(Self {
            actor_id,
            clock,
            connection,
        })
    }

    pub fn transaction(&mut self) -> Result<Transaction<'_>> {
        Transaction::new(self)
    }

    #[inline]
    pub fn apply_migrations(&mut self) -> Result<()> {
        apply_migrations(self)?;
        Ok(())
    }

    fn row_to_task(row: &Row<'_>) -> Result<Task, rusqlite::Error> {
        let id = row.get::<_, Uuid>("id")?;
        let title = row.get::<_, Option<String>>("title")?;
        let entry = row.get::<_, Sql<Option<Date>>>("entry")?.value;
        let status = row.get::<_, Sql<Option<TaskStatus>>>("status")?.value;
        let priority = row.get::<_, Sql<Option<TaskPriority>>>("priority")?.value;
        let project = row.get::<_, Option<String>>("project")?;
        let modified = row.get::<_, Sql<Option<Date>>>("modified")?.value;
        let due = row.get::<_, Sql<Option<Date>>>("due")?.value;
        let wait = row.get::<_, Sql<Option<Date>>>("wait")?.value;
        let depends = row.get::<_, Sql<Vec<Uuid>>>("depends")?.value;
        let annoation_ids = row.get::<_, Option<Vec<u8>>>("annotation_ids")?;
        let annoation_entries = row.get::<_, Option<String>>("annotation_entries")?;
        let annoation_texts = row.get::<_, Option<String>>("annotation_texts")?;
        let uda_keys = row.get::<_, Option<String>>("uda_keys")?;
        let uda_values = row.get::<_, Option<String>>("uda_values")?;
        let tags = row
            .get::<_, Option<String>>("tags")?
            .map(|tags| tags.split('\0').map(String::from).collect::<Vec<_>>())
            .unwrap_or_default();

        let mut annotations = Vec::new();
        if let (Some(ids), Some(entries), Some(text)) =
            (annoation_ids, annoation_entries, annoation_texts)
        {
            for (id, (entry, description)) in ids
                .chunks_exact(16)
                .zip(entries.split(' ').zip(text.split('\0')))
            {
                let id = Uuid::from_bytes(id.try_into().expect("should be 16 bytes"));
                let entry = entry.parse::<i64>().expect("should be a valid integer");
                annotations.push(Annotation {
                    id,
                    entry: DateTime::from_timestamp_millis(entry)
                        .expect("should be a valid entry timestamp"),
                    text: description.into(),
                });
            }
        }

        let mut udas = Vec::new();
        if let (Some(keys), Some(values)) = (uda_keys, uda_values) {
            for (key, value) in keys.split('\0').zip(values.split('\0')) {
                udas.push(Uda {
                    key: key.into(),
                    value: value.into(),
                });
            }
        }

        Ok(Task {
            id,
            entry,
            status,
            title,
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
        self.tasks_by_status(&[TaskStatus::Pending, TaskStatus::Done, TaskStatus::Deleted].into())
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
                let title = Some(title.to_lowercase());
                let mut tasks = self.tasks_by_status(status)?;
                tasks.retain(|task| task.title.as_deref().map(str::to_uppercase) == title);
                tasks.truncate(limit.unwrap_or(u32::MAX) as usize);
                tasks.sort_unstable_by(|a, b| b.urgency().total_cmp(&a.urgency()));
                Ok(tasks)
            }
        }
    }

    pub fn tasks_by_status(&mut self, status: &HashSet<TaskStatus>) -> Result<Vec<Task>> {
        let mut tasks = Vec::new();
        let mut sql = self.connection.prepare_cached(SQL_ALL)?;
        let statys_array = Rc::new(
            status
                .iter()
                .copied()
                .map(task_status_to_sql)
                .map(rusqlite::types::Value::from)
                .collect::<Vec<_>>(),
        );
        let task_iter = sql.query_map([statys_array], Self::row_to_task)?;
        for task in task_iter {
            let task = task?;
            if let Some(task_status) = &task.status
                && status.contains(task_status)
            {
                tasks.push(task);
            }
        }
        tasks.sort_unstable_by(|a, b| b.urgency().total_cmp(&a.urgency()));
        Ok(tasks)
    }

    pub fn tasks_by_title(&mut self, title: &str) -> Result<Vec<Task>> {
        let title = title.to_lowercase();
        let mut tasks = Vec::new();
        let mut sql = self.connection.prepare_cached(SQL_ALL)?;
        let task_iter = sql.query_map([], Self::row_to_task)?;
        for task in task_iter {
            let task = task?;
            if task
                .title
                .as_ref()
                .is_some_and(|task_title| task_title.to_lowercase() == title)
            {
                tasks.push(task);
            }
        }
        tasks.sort_unstable_by(|a, b| b.urgency().total_cmp(&a.urgency()));
        Ok(tasks)
    }

    pub fn task_by_id(&mut self, id: Uuid) -> Result<Option<Task>> {
        self.connection
            .query_row(SQL_BY_ID, (id,), Self::row_to_task)
            .optional()
            .map_err(Into::into)
    }

    pub fn add_backend(&mut self, backend: &BackendRecord) -> Result<()> {
        let transaction = self.connection.transaction()?;

        transaction.execute(
            indoc! {"
                INSERT INTO backend_table (
                    id,
                    name,
                    enabled
                )
                VALUES (?1, ?2, ?3)
            "},
            (backend.id.as_bytes(), &backend.name, &backend.enabled),
        )?;

        for (name, value) in &backend.config.fields {
            transaction.execute(
                indoc! {"
                    INSERT INTO backend_config_table (
                        backend_id,
                        name,
                        type,
                        value
                    ) VALUES (?1, ?2, ?3, ?4)
                "},
                (
                    backend.id.as_bytes(),
                    name,
                    value.as_type_name(),
                    value.as_value_boxed_slice(),
                ),
            )?;
        }
        transaction.commit()?;
        Ok(())
    }

    pub fn backends(&mut self) -> Result<Vec<BackendRecord>> {
        let transaction = self.connection.transaction()?;
        let mut backend_sql = transaction.prepare(indoc! {"
            SELECT
                id,
                name,
                enabled
            FROM
                backend_table
        "})?;

        let rows = backend_sql.query_map((), |row| {
            let id = row.get::<_, Uuid>("id")?;

            let mut backend_config_sql = transaction.prepare_cached(indoc! {"
                SELECT
                    name,
                    type,
                    value
                FROM
                    backend_config_table
                WHERE
                    backend_id = ?1
            "})?;
            let rows = backend_config_sql.query_map((id,), |row| {
                let name = row.get::<_, Box<str>>("name")?;
                let typ = row.get::<_, Box<str>>("type")?;

                Ok((
                    name,
                    match typ.as_ref() {
                        typ @ ("string" | "url") => {
                            let value = row.get::<_, Vec<u8>>("value")?;
                            let string = String::from_utf8(value)
                                .map_err(|e| rusqlite::Error::Utf8Error(e.utf8_error()))?;
                            if typ == "string" {
                                Value::String(string.into_boxed_str())
                            } else {
                                Value::Url(Url::parse(&string).map_err(|e| {
                                    rusqlite::types::FromSqlError::Other(Box::new(e))
                                })?)
                            }
                        }
                        "uuid" => Value::Uuid(row.get::<_, Uuid>("value")?),
                        "bytes" => Value::Bytes(row.get::<_, Vec<u8>>("value")?.into_boxed_slice()),
                        _ => todo!(),
                    },
                ))
            })?;

            let mut fields = HashMap::new();
            for field in rows {
                let (name, value) = field?;
                fields.insert(name, value);
            }
            Ok(BackendRecord {
                id: row.get::<_, Uuid>("id")?,
                name: row.get::<_, Box<str>>("name")?,
                enabled: row.get::<_, bool>("enabled")?,
                config: Config { fields },
            })
        })?;
        let mut backends = Vec::new();
        for backend in rows {
            backends.push(backend?);
        }

        Ok(backends)
    }

    pub fn toggle_backend(&mut self, id: Uuid) -> Result<()> {
        self.connection.execute(
            indoc! {"
                UPDATE backend_table
                SET
                    enabled = (CASE WHEN enabled = 0 THEN 1 ELSE 0 END)
                WHERE
                    id = ?1
            "},
            (id.as_bytes(),),
        )?;
        Ok(())
    }

    pub fn update_backend(&mut self, backend: &BackendRecord) -> Result<()> {
        let transaction = self.connection.transaction()?;

        // TODO: Maybe instead of deleting them all,
        // figure out a nice way to delete only the tags that are old.
        transaction.execute(
            "DELETE FROM backend_config_table WHERE backend_id = ?1",
            (backend.id,),
        )?;
        let mut sql = transaction
            .prepare_cached("INSERT INTO backend_config_table (backend_id, name, type, value) VALUES (?1, ?2, ?3, ?4) ON CONFLICT DO NOTHING")?;
        for (name, value) in &backend.config.fields {
            sql.execute((
                backend.id,
                name,
                value.as_type_name(),
                value.as_value_boxed_slice(),
            ))?;
        }
        drop(sql);

        transaction.execute(
            indoc! {"
                UPDATE backend_table
                SET
                    name = ?2,
                    enabled = ?3
                WHERE
                    id = ?1
            "},
            (backend.id.as_bytes(), &backend.name, &backend.enabled),
        )?;

        transaction.commit()?;
        Ok(())
    }

    pub fn delete_backend(&mut self, id: Uuid) -> Result<()> {
        self.connection
            .execute("DELETE FROM backend_table WHERE id = ?1", (id.as_bytes(),))?;
        Ok(())
    }
}
