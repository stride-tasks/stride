use std::{collections::HashSet, path::PathBuf, sync::Mutex};

use flutter_rust_bridge::frb;
use stride_backend::{
    Backend, Error as BackendError,
    git::{GitBackend, config::GitConfig, encryption_key::EncryptionKey, ssh_key::SshKey},
};
use stride_core::{
    event::TaskQuery,
    task::{Task, TaskStatus},
};
use stride_database::Database;
use uuid::Uuid;

use crate::RustError;

use super::{
    filter::Filter,
    settings::{Settings, application_support_path, ssh_keys},
};

#[frb(opaque)]
#[derive(Debug)]
pub struct Repository {
    uuid: Uuid,
    pub root_path: PathBuf,
    pub db: Mutex<Database>,
}

impl Repository {
    #[flutter_rust_bridge::frb(sync)]
    pub fn open(uuid: Uuid) -> Result<Self, RustError> {
        let root_path = application_support_path()
            .join("repository")
            .join(uuid.to_string());
        std::fs::create_dir_all(&root_path)?;
        let db_path = root_path.join("db.sqlite");
        let mut db = Database::open(&db_path).map_err(Into::<stride_database::Error>::into)?;
        db.apply_migrations()?;
        Ok(Self {
            uuid,
            db: db.into(),
            root_path,
        })
    }

    pub fn all_tasks(&mut self, filter: Filter) -> Result<Vec<Task>, RustError> {
        let search = filter.search.to_lowercase();
        let mut tasks = self.db.lock().unwrap().tasks_by_status(&filter.status)?;
        tasks.retain(|task| task.title.to_lowercase().contains(&search));
        Ok(tasks)
    }

    pub fn insert_task(&mut self, task: &Task) -> Result<(), RustError> {
        self.db.lock().unwrap().insert_task(task)?;
        Ok(())
    }

    pub fn update_task(&mut self, task: &Task) -> Result<(), RustError> {
        self.db.lock().unwrap().update_task(task)?;
        Ok(())
    }

    pub fn purge_task_by_id(&mut self, id: Uuid) -> Result<Option<Task>, RustError> {
        Ok(self.db.lock().unwrap().purge_task_by_id(id)?)
    }

    pub fn tasks_by_status(
        &mut self,
        status: &HashSet<TaskStatus>,
    ) -> Result<Vec<Task>, RustError> {
        Ok(self.db.lock().unwrap().tasks_by_status(status)?)
    }

    pub fn task_by_id(&mut self, id: Uuid) -> Result<Option<Task>, RustError> {
        Ok(self.db.lock().unwrap().task_by_id(id)?)
    }

    pub fn task_query(&mut self, query: &TaskQuery) -> Result<Vec<Task>, RustError> {
        Ok(self.db.lock().unwrap().task_query(query)?)
    }

    pub fn sync(&mut self) -> Result<(), RustError> {
        let mut settings = Settings::get();
        let specification = settings
            .repositories
            .iter_mut()
            .find(|repository| repository.uuid == self.uuid)
            .unwrap();

        let encryption = if let Some(encrpytion) = &specification.encryption {
            encrpytion
        } else {
            let reference = specification
                .encryption
                .get_or_insert(EncryptionKey::generate());
            reference
        };

        let Some(ssh_key_uuid) = specification.ssh_key_uuid else {
            return Err(BackendError::NoSshKeysProvided.into());
        };

        let ssh_keys = ssh_keys()?;
        let Some(ssh_key) = ssh_keys.iter().find(|ssh_key| ssh_key.uuid == ssh_key_uuid) else {
            return Err(BackendError::NoSshKeysProvided.into());
        };

        let git_backend_path = self.root_path.join("backend").join("git");
        std::fs::create_dir_all(&git_backend_path)?;

        let config = GitConfig {
            root_path: git_backend_path,
            author: specification.author.clone().into_boxed_str(),
            email: specification.author.clone().into_boxed_str(),
            branch: specification.branch.clone().into_boxed_str(),
            origin: specification.origin.clone().into_boxed_str(),
            encryption_key: encryption.clone(),
            ssh_key: SshKey {
                uuid: ssh_key.uuid,
                public_key: ssh_key.public_key.clone(),
                public_path: ssh_key.public_path.clone(),
                private_path: ssh_key.private_path.clone(),
            },
        };

        Settings::save(settings.clone())?;

        let mut backend = GitBackend::new(config)?;
        self.db.clear_poison();
        backend.sync(self.db.get_mut().expect("poison valued cleared"))?;
        Ok(())
    }
}
