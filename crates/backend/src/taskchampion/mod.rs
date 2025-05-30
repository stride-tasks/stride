use std::{mem, path::PathBuf};

use stride_core::task::Task;
use stride_database::Database;
use taskchampion::{Operations, StorageConfig};
use uuid::Uuid;

use super::Backend;

// Re-export this, to allow API users to access the ServerConfig, without depending on
// `taskchampion`
pub use taskchampion::ServerConfig;

use crate::Result;

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
    source: taskchampion::Replica,
    operations: Operations,
    server: Box<dyn taskchampion::Server>,

    /// This will avoid generating snapshots for the server, and leave other clients (for example
    /// on a desktop) the freedom to do so.
    constraint_environment: bool,
}

impl TaskchampionBackend {
    pub fn new(config: TaskchampionConfig) -> Result<Self> {
        let storage = StorageConfig::OnDisk {
            taskdb_dir: config.root_path,
            create_if_missing: true,
            access_mode: taskchampion::storage::AccessMode::ReadWrite,
        }
        .into_storage()?;
        let source = taskchampion::Replica::new(storage);

        let server_config = ServerConfig::Remote {
            url: config.url,
            client_id: config.client_id,
            encryption_secret: config.encryption_secret,
        };

        Ok(Self {
            source,
            operations: Operations::new(),
            server: server_config.into_server()?,
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

    pub fn add(&mut self, task: Task) -> Result<()> {
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

        let mut champion_task = self.source.create_task(task.uuid, &mut self.operations)?;

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

    pub fn commit(&mut self) -> Result<()> {
        let operations = mem::take(&mut self.operations);

        if let Err(err) = self.source.commit_operations(operations.clone()) {
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
}

impl Backend for TaskchampionBackend {
    fn sync(&mut self, db: &mut Database) -> Result<()> {
        for task in db.all_tasks()? {
            self.add(task)?;
        }

        /* PERF(@bpeetz): We should probably not always force a commit. <2024-10-26> */
        self.commit()?;

        self.source
            /* TODO(@bpeetz): It would be wonderful, if we could add the server URL to this error
             * message. But the [`taskchampion::Server`] trait does not provide us this
             * information.   <2024-10-26> */
            .sync(&mut self.server, self.constraint_environment)?;

        Ok(())
    }
}
