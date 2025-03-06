use std::{mem, path::Path};

use chrono::Utc;
use flutter_rust_bridge::frb;
use stride_core::{
    event::TaskQuery,
    task::{Task, TaskStatus},
};
use taskchampion::{Operations, StorageConfig};

use super::StrideRepository;

// Re-export this, to allow API users to access the ServerConfig, without depending on
// `taskchampion`
pub use taskchampion::ServerConfig;

#[frb(ignore)]
#[allow(missing_debug_implementations)] /* [`taskchampion::Replica`] does not implement [`Debug`] */
pub struct Replica {
    source: taskchampion::Replica,
    operations: Operations,
    server: Box<dyn taskchampion::Server>,

    /// This will avoid generating snapshots for the server, and leave other clients (for example
    /// on a desktop) the freedom to do so.
    constraint_environment: bool,
}

impl Replica {
    pub fn new(
        storage_dir: &Path,
        server_config: ServerConfig,
        constraint_environment: bool,
    ) -> Result<Self, crate::RustError> {
        let storage = StorageConfig::OnDisk {
            taskdb_dir: storage_dir.to_path_buf(),
            create_if_missing: true,
            access_mode: taskchampion::storage::AccessMode::ReadWrite,
        }
        .into_storage()?;
        let source = taskchampion::Replica::new(storage);

        Ok(Self {
            source,
            operations: Operations::new(),
            server: server_config.into_server()?,
            constraint_environment,
        })
    }
}

impl StrideRepository for Replica {
    fn unload(&mut self) {
        todo!()
    }

    fn add(&mut self, task: Task) -> Result<(), crate::RustError> {
        // Theoretically we need to set all the keys below:
        // add_annotation
        // add_dependency
        // add_tag
        // set_description [x]
        // set_due
        // set_entry [x]
        // set_modified [x]
        // set_priority
        // set_status [x]
        // set_uda
        // set_value
        // set_wait
        /* TODO(@bpeetz): Actually set all of these keys. <2024-10-26> */

        let now = Utc::now();
        let mut champion_task = self.source.create_task(task.uuid, &mut self.operations)?;

        champion_task.set_modified(now, &mut self.operations)?;
        champion_task.set_description(task.title, &mut self.operations)?;
        champion_task.set_status(task.status.into(), &mut self.operations)?;
        champion_task.set_entry(Some(now), &mut self.operations)?;

        Ok(())
    }

    fn remove_by_uuid(&mut self, _uuid: &uuid::Uuid) -> Result<Option<Task>, crate::RustError> {
        todo!()
    }

    fn remove_by_task(&mut self, _task: &Task) -> Result<bool, crate::RustError> {
        todo!()
    }

    fn task_by_uuid(&mut self, _uuid: &uuid::Uuid) -> Result<Option<Task>, crate::RustError> {
        todo!()
    }

    fn tasks_with_filter(
        &mut self,
        filter: &crate::api::filter::Filter,
    ) -> Result<Vec<Task>, crate::RustError> {
        let search = filter.search.to_lowercase();
        let mut result = Vec::new();

        for task in self
            .source
            .all_tasks()?
            .into_values()
            .filter(|task| {
                filter
                    .status
                    .contains(&Into::<TaskStatus>::into(task.get_status()))
            })
            .filter(|task| task.get_description().to_lowercase().contains(&search))
        {
            result.push(Into::<Task>::into(task));
        }

        Ok(result)
    }

    fn update(&mut self, _task: &Task) -> Result<bool, crate::RustError> {
        todo!()
    }

    fn change_category(
        &mut self,
        _task: &Task,
        _status: TaskStatus,
    ) -> Result<bool, crate::RustError> {
        todo!()
    }

    fn sync(&mut self) -> Result<(), crate::RustError> {
        /* PERF(@bpeetz): We should probably not always force a commit. <2024-10-26> */
        self.commit()?;

        self.source
            /* TODO(@bpeetz): It would be wonderful, if we could add the server URL to this error
             * message. But the [`taskchampion::Server`] trait does not provide us this
             * information.   <2024-10-26> */
            .sync(&mut self.server, self.constraint_environment)?;

        Ok(())
    }

    fn clear(&mut self) -> Result<(), crate::RustError> {
        todo!()
    }

    fn export(&mut self) -> Result<String, crate::RustError> {
        todo!()
    }

    fn import(&mut self, _content: &str) -> Result<(), crate::RustError> {
        todo!()
    }

    fn commit(&mut self) -> Result<(), crate::RustError> {
        let operations = mem::take(&mut self.operations);

        if let Err(err) = self.source.commit_operations(operations.clone()) {
            // On error, restore the taken operations.
            // Otherwise, these would be silently dropped.
            self.operations = operations;

            return Err(err.into());
        };

        assert!(
            self.operations.is_empty(),
            "No operations should persist on a successful commit."
        );

        Ok(())
    }
    fn query(&mut self, _query: &TaskQuery) -> Result<Vec<Task>, crate::RustError> {
        todo!()
    }
}
