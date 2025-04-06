use std::{collections::HashSet, path::PathBuf, sync::Mutex};

use flutter_rust_bridge::frb;
use stride_core::{
    event::TaskQuery,
    task::{Task, TaskStatus},
};
use stride_database::Database;
use uuid::Uuid;

use crate::RustError;

use super::{filter::Filter, settings::application_support_path};

#[frb(opaque)]
#[derive(Debug)]
pub struct Repository {
    root_path: PathBuf,
    db_path: PathBuf,
    db: Mutex<Database>,
}

impl Repository {
    #[flutter_rust_bridge::frb(sync)]
    pub fn open(uuid: Uuid) -> Result<Self, RustError> {
        let root_path = application_support_path()
            .join("repository")
            .join(uuid.to_string());
        let db_path = root_path.join("db.sqlite");
        let mut db = Database::open(&db_path).map_err(Into::<stride_database::Error>::into)?;
        db.apply_migrations()?;
        Ok(Self {
            db: db.into(),
            db_path,
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
}
