use std::sync::{Mutex, MutexGuard};
use std::{path::PathBuf, sync::Arc};
use stride_core::state::KnownPaths;
use stride_crdt::actor::ActorId;
use stride_crdt::hlc::{Clock, SystemTimeProvider};
use uuid::Uuid;

use stride_database::Database;

use crate::{Engine, Result};

#[derive(Debug)]
pub struct Repository {
    pub uuid: Uuid,
    pub root_path: PathBuf,
    pub db: Mutex<Database>,
}

impl Repository {
    pub fn open(id: Uuid, known_paths: &KnownPaths) -> Result<Self> {
        let root_path = known_paths.support.join("repository").join(id.to_string());
        std::fs::create_dir_all(&root_path)?;

        let time_provider = Arc::new(SystemTimeProvider::default());
        let clock = Clock::new(time_provider);
        let db_path = root_path.join("db.sqlite");
        let mut db = Database::open(&db_path, ActorId::new(id), clock)?;
        db.apply_migrations()?;

        Ok(Self {
            uuid: id,
            root_path,
            db: Mutex::new(db),
        })
    }

    pub fn lock_database(&self) -> MutexGuard<'_, Database> {
        self.db.lock().unwrap_or_else(|err| err.into_inner())
    }

    pub fn sync(&self, engine: &Arc<Engine>) -> Result<Vec<crate::api::TaskChange>> {
        let mut db = self.lock_database();
        let diff = engine
            .backends()
            .sync_all(self.uuid, &mut db, &engine.known_paths(), engine)?;

        Ok(db.transaction()?.task_changes_from_diff(&diff)?)
    }
}
