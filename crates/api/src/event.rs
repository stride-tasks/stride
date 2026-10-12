use crate::{Task, TaskQuery};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum NetworkRequestType {
    Get,
}

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
