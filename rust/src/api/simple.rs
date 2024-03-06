use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::{Ok, Result};
use flutter_rust_bridge::frb;
use uuid::Uuid;

use crate::task::{Task, TaskBuilder};

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[frb(opaque)]
pub struct TaskRepository {
    path: PathBuf,
    tasks: Vec<Task>,
}

impl TaskRepository {
    const PENDING_DATA_FILENAME: &'static str = "pending.data";
    // const COMPLETE_DATA_FILENAME: &'static str = "complete.data";
    // const DELETED_DATA_FILENAME: &'static str = "deleted.data";

    pub fn load(path: String) -> Result<Self> {
        let path = PathBuf::from(path);
        let mut tasks = Vec::new();

        let pending_filepath = path.join(Self::PENDING_DATA_FILENAME);
        if !pending_filepath.exists() {
            return Ok(Self {
                path,
                tasks: Vec::new(),
            });
        }
        let file = File::open(pending_filepath)?;
        let buf = BufReader::new(file);
        for line in buf.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            let task = serde_json::from_str(&line)?;

            tasks.push(task);
        }

        Ok(Self { path, tasks })
    }

    fn save(&mut self) -> Result<()> {
        let mut content = String::new();
        for task in &self.tasks {
            content += &serde_json::to_string(task)?;
            content.push('\n');
        }

        std::fs::write(self.path.join(Self::PENDING_DATA_FILENAME), content.trim())?;

        Ok(())
    }

    pub fn add(&mut self, task: Task) -> Result<()> {
        self.tasks.push(task);

        self.save()
    }

    pub fn task_by_uuid(&mut self, uuid: Uuid) -> Option<Task> {
        self.tasks.iter().find(|task| task.uuid == uuid).cloned()
    }

    pub fn tasks_by_description(&mut self, search: String) -> Vec<Task> {
        self.tasks
            .iter()
            .filter(|task| task.description.contains(&search))
            .cloned()
            .collect()
    }

    pub fn update(&mut self, task: Task) -> Result<bool> {
        let current = self
            .tasks
            .iter_mut()
            .find(|element| element.uuid == task.uuid);
        let Some(current) = current else {
            return Ok(false);
        };
        *current = task;

        self.save()?;
        Ok(true)
    }

    pub fn delete(&mut self, uuid: Uuid) -> Result<()> {
        self.tasks.retain(|task| task.uuid != uuid);
        self.save()?;
        Ok(())
    }

    pub fn tasks(&self) -> Vec<Task> {
        self.tasks.clone()
    }
}

impl Task {
    #[frb(sync)]
    pub fn new(description: String) -> Self {
        TaskBuilder::with_description(description)
            .build()
            .expect("All other fields are default initialized")
    }
}
