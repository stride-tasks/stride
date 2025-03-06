use std::collections::HashSet;

use crate::task::{Task, TaskStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    TaskQuery {
        tasks: Vec<Task>,
    },
    TaskSync,
    NetworkResponse {
        host: String,
        content: Box<[u8]>,
    },
    Timer {
        interval: u32,
    },
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
    pub fn task_query(tasks: Vec<Task>) -> Self {
        HostEvent::TaskQuery { tasks }
    }

    /// flutter_rust_bridge:sync
    #[must_use]
    pub fn task_sync() -> Self {
        HostEvent::TaskSync
    }

    /// flutter_rust_bridge:sync
    #[must_use]
    pub fn network_response(host: String, content: Vec<u8>) -> Self {
        HostEvent::NetworkResponse {
            host,
            content: content.into_boxed_slice(),
        }
    }

    /// flutter_rust_bridge:sync
    #[must_use]
    pub fn timer(interval: u32) -> Self {
        HostEvent::Timer { interval }
    }
}

/// flutter_rust_bridge:non_opaque
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum NetworkRequestType {
    Get,
}

/// flutter_rust_bridge:non_opaque
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TaskQuery {
    Uuid {
        uuid: Uuid,
    },
    Title {
        title: String,
        status: HashSet<TaskStatus>,
        limit: Option<u32>,
    },
}

/// flutter_rust_bridge:non_opaque
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PluginEvent {
    TaskCreate {
        task: Task,
    },
    TaskModify {
        task: Task,
    },
    TaskQuery {
        query: TaskQuery,
    },
    TaskSync,
    NetworkRequest {
        #[serde(rename = "type")]
        ty: NetworkRequestType,
        host: String,
    },
}
