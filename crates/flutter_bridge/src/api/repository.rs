use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use chrono::Utc;
use flutter_rust_bridge::frb;
use stride_core::{
    event::TaskQuery,
    task::{Task, TaskStatus},
};
use stride_crdt::{
    actor::ActorId,
    hlc::{Clock, SystemTimeProvider},
};
use stride_database::Database;
use uuid::Uuid;

use crate::{
    RustError,
    api::{filter::Filter, settings::application_support_path},
};

#[frb(opaque)]
#[derive(Debug)]
pub struct Repository {
    pub(crate) db: Mutex<Database>,
}

impl Repository {
    #[flutter_rust_bridge::frb(sync)]
    pub fn open(uuid: Uuid) -> Result<Self, RustError> {
        let root_path = application_support_path()
            .join("repository")
            .join(uuid.to_string());
        std::fs::create_dir_all(&root_path)?;

        let time_provider = Arc::new(SystemTimeProvider::default());
        let clock = Clock::new(time_provider);
        let db_path = root_path.join("db.sqlite");
        let mut db = Database::open(&db_path, ActorId::new(uuid), clock)
            .map_err(Into::<stride_database::Error>::into)?;
        db.apply_migrations()?;

        Ok(Self { db: db.into() })
    }

    pub fn all_tasks(&mut self, filter: &Filter) -> Result<Vec<Task>, RustError> {
        let search = filter.search.to_lowercase();
        self.db.clear_poison();
        let mut tasks = self.db.lock().unwrap().tasks_by_status(&filter.status)?;
        tasks.retain(|task| {
            task.title
                .as_ref()
                .is_some_and(|title| title.to_lowercase().contains(&search))
        });
        tasks.sort_unstable_by(|a, b| b.urgency().total_cmp(&a.urgency()));
        Ok(tasks)
    }

    pub fn insert_task(&mut self, task: &Task) -> Result<(), RustError> {
        self.db.clear_poison();
        let mut db = self.db.lock().unwrap();
        let mut transaction = db.transaction()?;
        transaction.insert_task(task)?;
        transaction.commit()?;
        Ok(())
    }

    pub fn update_task(&mut self, task: &Task) -> Result<(), RustError> {
        let mut task = task.clone();
        task.modified = Some(Utc::now());

        self.db.clear_poison();
        let mut db = self.db.lock().unwrap();
        let mut transaction = db.transaction()?;
        transaction.update_task_with(task.id, |_| Ok(task))?;
        transaction.commit()?;
        Ok(())
    }

    pub fn purge_task_by_id(&mut self, id: Uuid) -> Result<Option<Task>, RustError> {
        self.db.clear_poison();
        let mut db = self.db.lock().unwrap();
        let mut transaction = db.transaction()?;
        transaction.delete_task(id)?;
        transaction.commit()?;
        // TODO: maybe this should return Ok(())
        Ok(None)
    }

    pub fn tasks_by_status(
        &mut self,
        status: &HashSet<TaskStatus>,
    ) -> Result<Vec<Task>, RustError> {
        self.db.clear_poison();
        let mut tasks = self.db.lock().unwrap().tasks_by_status(status)?;
        tasks.sort_unstable_by(|a, b| b.urgency().total_cmp(&a.urgency()));
        Ok(tasks)
    }

    pub fn task_by_id(&mut self, id: Uuid) -> Result<Option<Task>, RustError> {
        self.db.clear_poison();
        Ok(self.db.lock().unwrap().task_by_id(id)?)
    }

    pub fn task_query(&mut self, query: &TaskQuery) -> Result<Vec<Task>, RustError> {
        self.db.clear_poison();
        Ok(self.db.lock().unwrap().task_query(query)?)
    }
}
