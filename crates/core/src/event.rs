use crate::task::Task;
use serde::{Deserialize, Serialize};

/// flutter_rust_bridge:ignore
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PluginEvent {
    TaskCreate { task: Task },
    TaskModify { task: Task },
    TaskSync,
}
