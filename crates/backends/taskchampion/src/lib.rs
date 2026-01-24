//! Stride's taskchampion backend implementations.

#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use std::{
    mem,
    path::{Path, PathBuf},
};

use stride_backend::{Backend, BackendHandler};
use stride_core::{
    backend::{BytesCategory, Config, Schema, SchemaValue},
    state::KnownPaths,
    task::Task,
};
use stride_database::Database;
use taskchampion::{Operations, Replica, ServerConfig, SqliteStorage, storage::AccessMode};
use uuid::Uuid;

pub mod error;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Debug, Clone, Copy)]
struct Handler;

impl BackendHandler for Handler {
    fn name(&self) -> Box<str> {
        "task-champion".into()
    }

    fn config_schema(&self) -> Schema {
        Schema::builder(self.name())
            .field(
                "url",
                "Server URL",
                SchemaValue::Url { default: None },
                false,
            )
            .field(
                "client_id",
                "Client ID",
                SchemaValue::Uuid { default: None },
                false,
            )
            .field(
                "encryption_secret",
                "Encryption Secret",
                SchemaValue::Bytes {
                    default: None,
                    min: None,
                    max: None,
                    category: Some(BytesCategory::Password),
                    generator: None,
                },
                true,
            )
            .build()
    }

    fn create(
        &self,
        schema: &Config,
        path: &Path,
        _known_paths: &KnownPaths,
    ) -> stride_backend::Result<Box<dyn Backend>> {
        let config = TaskchampionConfig {
            root_path: path.to_path_buf(),
            url: schema.url_value("url")?.to_string(),
            client_id: schema.uuid_value("client_id")?,
            encryption_secret: schema.bytes_value("encryption_secret")?.to_vec(),
            constraint_environment: true,
        };

        let runtime = tokio::runtime::Runtime::new().expect("cannot create tokio runtime");
        let this = runtime.block_on(TaskchampionBackend::new(config))?;

        Ok(Box::new(this))
    }
}

#[derive(Debug)]
pub struct TaskchampionConfig {
    pub root_path: PathBuf,
    pub url: String,
    pub client_id: Uuid,
    pub encryption_secret: Vec<u8>,

    pub constraint_environment: bool,
}

#[allow(missing_debug_implementations)] /* [`taskchampion::Replica`] does not implement [`Debug`] */
pub struct TaskchampionBackend {
    source: Replica<SqliteStorage>,
    operations: Operations,
    server: Box<dyn taskchampion::Server>,

    /// This will avoid generating snapshots for the server, and leave other clients (for example
    /// on a desktop) the freedom to do so.
    constraint_environment: bool,
}

impl TaskchampionBackend {
    pub async fn new(config: TaskchampionConfig) -> Result<Self> {
        std::fs::create_dir_all(&config.root_path)?;

        // Create a new Replica, storing data on disk.
        let storage = SqliteStorage::new(config.root_path, AccessMode::ReadWrite, true).await?;
        let source = Replica::new(storage);

        // Set up a local, on-disk server.
        let server_config = ServerConfig::Remote {
            url: config.url,
            client_id: config.client_id,
            encryption_secret: config.encryption_secret,
        };
        let server = server_config.into_server().await?;

        Ok(Self {
            source,
            operations: Operations::new(),
            server,
            constraint_environment: config.constraint_environment,
        })
    }

    // fn tasks_with_filter(&mut self, filter: &crate::api::filter::Filter) -> Result<Vec<Task>> {
    //     let search = filter.search.to_lowercase();
    //     let mut result = Vec::new();
    //     for task in self
    //         .source
    //         .all_tasks()?
    //         .into_values()
    //         .filter(|task| {
    //             filter
    //                 .status
    //                 .contains(&Into::<TaskStatus>::into(task.get_status()))
    //         })
    //         .filter(|task| task.get_description().to_lowercase().contains(&search))
    //     {
    //         result.push(Into::<Task>::into(task));
    //     }
    //     Ok(result)
    // }

    pub async fn add(&mut self, task: Task) -> Result<()> {
        // Theoretically we need to set all the keys below:
        // add_annotation   [x]
        // add_dependency
        // add_tag          [x]
        // set_description  [x]
        // set_due          [x]
        // set_entry        [x]
        // set_modified     [x]
        // set_priority     [x]
        // set_status       [x]
        // set_uda
        // set_value
        // set_wait         [x]
        /* TODO(@bpeetz): Actually set all of these keys. <2024-10-26> */

        let mut champion_task = self
            .source
            .create_task(task.uuid, &mut self.operations)
            .await?;

        champion_task.set_entry(Some(task.entry), &mut self.operations)?;
        champion_task.set_description(task.title, &mut self.operations)?;
        champion_task.set_due(task.due, &mut self.operations)?;
        champion_task.set_wait(task.wait, &mut self.operations)?;
        if let Some(modified) = task.modified {
            champion_task.set_modified(modified, &mut self.operations)?;
        }
        champion_task.set_status(task.status.into(), &mut self.operations)?;
        for tag in &task.tags {
            champion_task.add_tag(
                &tag.parse::<taskchampion::Tag>().unwrap(),
                &mut self.operations,
            )?;
        }
        if let Some(priority) = task.priority {
            champion_task.set_priority(priority.as_str().to_string(), &mut self.operations)?;
        }
        for annotation in task.annotations {
            champion_task.add_annotation(
                taskchampion::Annotation {
                    entry: annotation.entry,
                    description: annotation.description,
                },
                &mut self.operations,
            )?;
        }

        Ok(())
    }

    pub async fn commit(&mut self) -> Result<()> {
        let operations = mem::take(&mut self.operations);

        if let Err(err) = self.source.commit_operations(operations.clone()).await {
            // On error, restore the taken operations.
            // Otherwise, these would be silently dropped.
            self.operations = operations;

            return Err(err.into());
        }

        assert!(
            self.operations.is_empty(),
            "No operations should persist on a successful commit."
        );

        Ok(())
    }

    async fn sync(&mut self, db: &mut Database) -> Result<(), stride_backend::Error> {
        for task in db.all_tasks()? {
            self.add(task).await?;
        }

        /* PERF(@bpeetz): We should probably not always force a commit. <2024-10-26> */
        self.commit().await?;

        /* TODO(@bpeetz): It would be wonderful, if we could add the server URL to this error
         * message. But the [`taskchampion::Server`] trait does not provide us this
         * information.   <2024-10-26> */
        self.source
            .sync(&mut self.server, self.constraint_environment)
            .await
            .map_err(error::Error::from)?;

        Ok(())
    }
}

impl Backend for TaskchampionBackend {
    fn handler() -> Box<dyn BackendHandler>
    where
        Self: Sized,
    {
        Box::new(Handler)
    }

    fn sync(&mut self, db: &mut Database) -> Result<(), stride_backend::Error> {
        let runtime = tokio::runtime::Runtime::new().expect("cannot create tokio runtime");
        runtime.block_on(self.sync(db))?;
        Ok(())
    }
}
