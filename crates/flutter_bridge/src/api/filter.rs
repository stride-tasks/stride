use std::collections::HashSet;

use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::task::TaskStatus;

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub uuid: Uuid,
    pub name: String,

    pub status: HashSet<TaskStatus>,
    pub search: String,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name: "Default".to_string(),
            status: [TaskStatus::Pending].into(),
            search: String::new(),
        }
    }
}

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterSelection {
    Predefined { uuid: Uuid },
    Temporary { filter: Filter },
}
