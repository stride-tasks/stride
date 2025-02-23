use crate::task::Task;
use serde::{Deserialize, Serialize};

/// flutter_rust_bridge:unignore
#[allow(clippy::doc_markdown)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HostEvent {
    TaskCreate {
        task: Option<Box<Task>>,
    },
    TaskModify {
        current: Option<Box<Task>>,
        previous: Option<Box<Task>>,
    },
    TaskSync,
}

impl HostEvent {
    /// flutter_rust_bridge:sync
    #[must_use]
    pub fn task_create(task: Option<Task>) -> Self {
        HostEvent::TaskCreate {
            task: task.map(Box::new),
        }
    }

    /// flutter_rust_bridge:sync
    #[must_use]
    pub fn task_modify(current: Option<Task>, previous: Option<Task>) -> Self {
        HostEvent::TaskModify {
            current: current.map(Box::new),
            previous: previous.map(Box::new),
        }
    }

    /// flutter_rust_bridge:sync
    #[must_use]
    pub fn task_sync() -> Self {
        HostEvent::TaskSync
    }
}

/// flutter_rust_bridge:non_opaque
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PluginEvent {
    TaskCreate { task: Task },
    TaskModify { task: Task },
    TaskSync,
}
