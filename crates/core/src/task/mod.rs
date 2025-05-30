#![allow(clippy::doc_markdown)]

use chrono::{DateTime, Utc};

pub mod annotation;
pub mod uda;

pub type Date = DateTime<Utc>;

pub use annotation::Annotation;
use serde::{Deserialize, Serialize};
pub use uda::Uda;
use uuid::Uuid;

#[cfg(test)]
mod tests;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TaskStatus {
    #[default]
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "recurring")]
    Recurring,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "complete")]
    Complete,
}

impl TaskStatus {
    #[must_use]
    pub fn is_pending(&self) -> bool {
        *self == TaskStatus::Pending
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TaskPriority {
    #[default]
    #[serde(rename = "H")]
    H,
    #[serde(rename = "M")]
    M,
    #[serde(rename = "L")]
    L,
}

impl TaskPriority {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            TaskPriority::H => "H",
            TaskPriority::M => "M",
            TaskPriority::L => "L",
        }
    }
}

/// flutter_rust_bridge:dart_metadata=("freezed")
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub uuid: Uuid,
    #[serde(default = "Utc::now")]
    pub entry: Date,
    #[serde(default)]
    #[serde(skip_serializing_if = "TaskStatus::is_pending")]
    pub status: TaskStatus,
    pub title: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub active: bool,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<Date>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<Date>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<Annotation>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TaskPriority>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<Date>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub depends: Vec<Uuid>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub udas: Vec<Uda>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            uuid: Uuid::now_v7(),
            status: TaskStatus::Pending,
            title: String::new(),
            entry: Utc::now(),
            active: false,
            modified: None,
            due: None,
            project: None,
            tags: Vec::new(),
            annotations: Vec::new(),
            priority: None,
            wait: None,
            depends: Vec::new(),
            udas: Vec::new(),
        }
    }
}

impl Task {
    /// flutter_rust_bridge:sync
    #[must_use]
    pub fn new(title: String) -> Self {
        Task {
            title,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_uuid(uuid: Uuid, title: String) -> Self {
        Task {
            uuid,
            title,
            ..Default::default()
        }
    }

    /// flutter_rust_bridge:sync
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn urgency(&self) -> f32 {
        const THREE_DAYS: i64 = 3 * 24 * 60 * 60;

        let mut urgency = 0.0;
        urgency += f32::from(self.active) * 15.0;
        if let Some(due) = self.due {
            let today = Utc::now();
            let delta = due - today;

            urgency += 1.0;

            let seconds = delta.num_seconds();
            if seconds < 0 {
                urgency += 11.0;
            } else if seconds <= THREE_DAYS {
                urgency += (seconds as f32 / THREE_DAYS as f32) * 11.0;
            }
        }
        if let Some(priority) = self.priority {
            match priority {
                TaskPriority::H => urgency += 6.0,
                TaskPriority::M => urgency += 3.0,
                TaskPriority::L => urgency += -3.0,
            }
        }
        urgency
    }
}

#[cfg(feature = "taskchampion")]
fn taskchampion_priority_to_task_status(priority: &str) -> Option<TaskPriority> {
    let priority = match priority.to_lowercase().as_str() {
        "l" | "low" => TaskPriority::L,
        "m" | "medium" => TaskPriority::M,
        "h" | "high" => TaskPriority::H,
        _ => return None,
    };
    Some(priority)
}

#[cfg(feature = "taskchampion")]
impl From<taskchampion::Task> for Task {
    fn from(v: taskchampion::Task) -> Self {
        /* TODO(@bpeetz): Remove the `None`s and `Vec`s with their actually conversion <2024-10-26> */
        Self {
            uuid: v.get_uuid(),
            entry: v.get_entry().unwrap_or_else(Utc::now),
            status: v.get_status().into(),
            title: v.get_description().into(),
            active: v.get_status() == taskchampion::Status::Pending,
            modified: v.get_modified(),
            due: v.get_due(),
            project: v.get_value("project").map(Into::into),
            tags: v
                .get_tags()
                .filter(taskchampion::Tag::is_user)
                .map(|tag| tag.as_ref().into())
                .collect(),
            annotations: v.get_annotations().map(Into::into).collect(),
            priority: taskchampion_priority_to_task_status(v.get_priority()),
            wait: v.get_wait(),
            depends: v.get_dependencies().collect(),
            // TODO: Remove use of deprecated function.
            #[allow(deprecated)]
            udas: v
                .get_udas()
                .map(|((namespace, key), value)| Uda {
                    namespace: namespace.into(),
                    key: key.into(),
                    value: value.into(),
                })
                .collect(),
        }
    }
}
#[cfg(feature = "taskchampion")]
impl From<taskchampion::Annotation> for Annotation {
    fn from(value: taskchampion::Annotation) -> Self {
        Self {
            entry: value.entry,
            description: value.description,
        }
    }
}
#[cfg(feature = "taskchampion")]
impl From<taskchampion::Status> for TaskStatus {
    fn from(value: taskchampion::Status) -> Self {
        match value {
            taskchampion::Status::Pending => Self::Pending,
            taskchampion::Status::Completed => Self::Complete,
            taskchampion::Status::Deleted => Self::Deleted,
            taskchampion::Status::Recurring => Self::Recurring,
            taskchampion::Status::Unknown(other) => {
                todo!("No implementation for unknown status: {other}")
            }
        }
    }
}
#[cfg(feature = "taskchampion")]
impl From<TaskStatus> for taskchampion::Status {
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::Pending => Self::Pending,

            /* FIXME(@bpeetz): This can't be correct <2024-10-26> */
            TaskStatus::Waiting => Self::Unknown("Waiting".to_owned()),

            TaskStatus::Recurring => Self::Recurring,
            TaskStatus::Deleted => Self::Deleted,
            TaskStatus::Complete => Self::Completed,
        }
    }
}
