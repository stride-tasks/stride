use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Mutex,
};

use flutter_rust_bridge::frb;
use stride_backend::{Backend, registry::Registry};
use stride_backend_git::GitBackend;
use stride_backend_taskchampion::TaskchampionBackend;
use stride_core::{
    backend::{BackendRecord as CoreBackendRecord, Config},
    event::TaskQuery,
    state::KnownPaths,
    task::{Task, TaskStatus},
};
use stride_database::Database;
use uuid::Uuid;

use crate::{
    RustError,
    api::{
        filter::Filter,
        settings::{application_cache_path, application_support_path},
    },
};

#[frb(opaque)]
#[derive(Debug)]
pub struct Repository {
    uuid: Uuid,
    pub(crate) root_path: PathBuf,
    pub(crate) db: Mutex<Database>,

    pub(crate) backend_registry: Registry,
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

        let mut backend_registry = Registry::new();
        backend_registry.insert(GitBackend::handler());
        backend_registry.insert(TaskchampionBackend::handler());
        Ok(Self {
            uuid,
            db: db.into(),
            root_path,
            backend_registry,
        })
    }

    pub fn remove(uuid: Uuid) -> Result<(), RustError> {
        let root_path = application_support_path()
            .join("repository")
            .join(uuid.to_string());
        std::fs::remove_dir_all(&root_path)?;
        Ok(())
    }

    pub fn all_tasks(&mut self, filter: &Filter) -> Result<Vec<Task>, RustError> {
        let search = filter.search.to_lowercase();
        self.db.clear_poison();
        let mut tasks = self.db.lock().unwrap().tasks_by_status(&filter.status)?;
        tasks.retain(|task| task.title.to_lowercase().contains(&search));
        Ok(tasks)
    }

    pub fn insert_task(&mut self, task: &Task) -> Result<(), RustError> {
        self.db.clear_poison();
        self.db.lock().unwrap().insert_task(task)?;
        Ok(())
    }

    pub fn update_task(&mut self, task: &Task) -> Result<(), RustError> {
        self.db.clear_poison();
        self.db.lock().unwrap().update_task(task)?;
        Ok(())
    }

    pub fn purge_task_by_id(&mut self, id: Uuid) -> Result<Option<Task>, RustError> {
        self.db.clear_poison();
        Ok(self.db.lock().unwrap().purge_task_by_id(id)?)
    }

    pub fn tasks_by_status(
        &mut self,
        status: &HashSet<TaskStatus>,
    ) -> Result<Vec<Task>, RustError> {
        self.db.clear_poison();
        Ok(self.db.lock().unwrap().tasks_by_status(status)?)
    }

    pub fn task_by_id(&mut self, id: Uuid) -> Result<Option<Task>, RustError> {
        self.db.clear_poison();
        Ok(self.db.lock().unwrap().task_by_id(id)?)
    }

    pub fn task_query(&mut self, query: &TaskQuery) -> Result<Vec<Task>, RustError> {
        self.db.clear_poison();
        Ok(self.db.lock().unwrap().task_query(query)?)
    }

    pub fn sync(&mut self) -> Result<(), RustError> {
        let known_paths = KnownPaths::new(application_support_path(), application_cache_path());

        let db = self.db.get_mut().unwrap();
        self.backend_registry
            .sync_all(self.uuid, db, &known_paths)?;

        Ok(())
    }

    pub fn undo(&self) -> Result<(), RustError> {
        self.db.lock().unwrap().undo(1)?;
        Ok(())
    }

    /// flutter_rust_bridge:ignore
    pub fn root_path(&self) -> &Path {
        &self.root_path
    }

    /// flutter_rust_bridge:ignore
    pub fn database(&self) -> &Mutex<Database> {
        &self.db
    }

    /// flutter_rust_bridge:ignore
    pub fn database_mut(&mut self) -> &mut Mutex<Database> {
        &mut self.db
    }

    pub fn backends(&self) -> Result<Vec<BackendRecord>, RustError> {
        let backends = self.database().lock().unwrap().backends()?;

        Ok(backends
            .into_iter()
            .map(|backend| BackendRecord {
                id: backend.id,
                name: backend.name.to_string(),
                enabled: backend.enabled,
                schema: serde_json::to_string_pretty(&GitBackend::handler().config_schema())
                    .expect("should not fail"),
                config: serde_json::to_string_pretty(&backend.config).expect("should not fail"),
            })
            .collect::<Vec<_>>())
    }

    pub fn toggle_backend(&self, id: Uuid) -> Result<(), RustError> {
        self.database().lock().unwrap().toggle_backend(id)?;
        Ok(())
    }

    pub fn update_backend(&self, backend: &BackendRecord) -> Result<(), RustError> {
        let config = serde_json::from_str(&backend.config).expect("invalid backend config json");
        self.database()
            .lock()
            .unwrap()
            .update_backend(&CoreBackendRecord {
                id: backend.id,
                name: backend.name.clone().into_boxed_str(),
                enabled: backend.enabled,
                config,
            })?;
        Ok(())
    }

    pub fn add_backend(&self, name: &str) -> Result<(), RustError> {
        let handler = self.backend_registry.get_or_error(name)?;
        let name = handler.name();
        self.database()
            .lock()
            .unwrap()
            .add_backend(&CoreBackendRecord {
                id: Uuid::now_v7(),
                name,
                enabled: false,
                config: Config::default(),
            })?;
        Ok(())
    }

    pub fn delete_backend(&self, id: Uuid) -> Result<(), RustError> {
        self.database().lock().unwrap().delete_backend(id)?;
        Ok(())
    }

    pub fn backend(&self, id: Uuid) -> Result<Option<BackendRecord>, RustError> {
        let backends = self.backends()?;

        Ok(backends
            .into_iter()
            .filter(|backend| backend.id == id)
            .nth(0))
    }

    pub fn backend_names(&self) -> Vec<String> {
        self.backend_registry.keys().map(Into::into).collect()
    }
}

#[derive(Debug)]
pub struct BackendRecord {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
    pub schema: String,
    pub config: String,
}
