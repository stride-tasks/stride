use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub mod annotation;

pub type Date = DateTime<Utc>;

pub use annotation::Annotation;
use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type TagIndex = u32;
pub type ProjectIndex = u32;
pub type PriorityIndex = u32;

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
    fn as_str(self) -> &'static str {
        match self {
            TaskPriority::H => "H",
            TaskPriority::M => "M",
            TaskPriority::L => "L",
        }
    }
}

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub uuid: Uuid,
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
    pub project: Option<ProjectIndex>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagIndex>,

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
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub uda: HashMap<String, String>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            uuid: Uuid::now_v7(),
            status: TaskStatus::Pending,
            title: String::new(),
            active: false,
            modified: None,
            due: None,
            project: None,
            tags: Vec::new(),
            annotations: Vec::new(),
            priority: None,
            wait: None,
            depends: Vec::new(),
            uda: HashMap::new(),
        }
    }
}

impl Task {
    #[must_use]
    pub fn entry(&self) -> Date {
        let timestamp = self
            .uuid
            .get_timestamp()
            .expect("uuid is v7 so this should not fail");
        let (secs, nsecs) = timestamp.to_unix();

        #[allow(clippy::cast_possible_wrap)]
        DateTime::from_timestamp(secs as i64, nsecs).expect("uuidv7 has a valid timestamp")
    }

    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn to_data(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(self.uuid.as_bytes());
        result.extend_from_slice(&(self.title.len() as u32).to_be_bytes());
        result.extend_from_slice(self.title.as_bytes());
        if self.active {
            result.push(b'A');
        }
        if let Some(modified) = self.modified {
            result.push(b'm');
            result.extend(&modified.timestamp_micros().to_be_bytes());
        }
        if let Some(due) = self.due {
            result.push(b'd');
            result.extend(&due.timestamp_micros().to_be_bytes());
        }
        if let Some(wait) = self.wait {
            result.push(b'w');
            result.extend(&wait.timestamp_micros().to_be_bytes());
        }
        if let Some(project) = self.project {
            result.push(b'p');
            result.extend_from_slice(&project.to_be_bytes());
        }
        if let Some(priority) = self.priority {
            result.push(b'r');
            result.push(priority.as_str().as_bytes()[0]);
        }
        for tag in &self.tags {
            result.push(b't');
            result.extend_from_slice(&tag.to_be_bytes());
        }
        for depend in &self.depends {
            result.push(b'n');
            result.extend_from_slice(depend.as_bytes());
        }
        if !self.annotations.is_empty() {
            todo!("Annotation not implemented")
        }
        if !self.uda.is_empty() {
            todo!("UDA not implemented")
        }
        result
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn from_data(input: &[u8]) -> Option<Task> {
        let (uuid_bytes, input) = input.split_at_checked(16)?;
        let uuid = Uuid::from_bytes(uuid_bytes.try_into().ok()?);
        let timestamp = uuid.get_timestamp()?;
        let (secs, nsecs) = timestamp.to_unix();
        let _entry = DateTime::from_timestamp(secs.try_into().ok()?, nsecs)?;

        let (title_len, input) = input.split_at_checked(size_of::<u32>())?;
        let title_len = u32::from_be_bytes(title_len.try_into().ok()?) as usize;

        let (title_bytes, input) = input.split_at_checked(title_len)?;

        let title = std::str::from_utf8(title_bytes).ok()?;

        let mut active = false;
        let mut modified = None;
        let mut due = None;
        let mut project = None;
        let mut priority = None;
        let mut wait = None;
        let mut depends = Vec::new();
        let mut tags = Vec::new();
        let mut i = 0;
        while i < input.len() {
            let Some(typ) = input.get(i).copied() else {
                break;
            };
            i += 1;
            match typ {
                b'A' => {
                    active = true;
                }
                b'm' | b'd' | b'w' => {
                    let timestamp = input.get(i..(i + size_of::<i64>()))?;
                    let timestamp = i64::from_be_bytes(timestamp.try_into().ok()?);
                    let datetime = DateTime::from_timestamp_micros(timestamp)?;

                    i += size_of::<i64>();

                    match typ {
                        b'm' => modified = Some(datetime),
                        b'd' => due = Some(datetime),
                        b'w' => wait = Some(datetime),
                        _ => unreachable!(),
                    }
                }
                b'n' => {
                    let uuid =
                        Uuid::from_bytes(input.get(i..i + size_of::<Uuid>())?.try_into().ok()?);
                    depends.push(uuid);
                    i += size_of::<Uuid>();
                }
                b't' => {
                    let tag =
                        u32::from_be_bytes(input.get(i..i + size_of::<u32>())?.try_into().ok()?);
                    i += size_of::<u32>();
                    tags.push(tag);
                }
                b'p' => {
                    let value =
                        u32::from_be_bytes(input.get(i..i + size_of::<u32>())?.try_into().ok()?);
                    i += size_of::<u32>();
                    project = Some(value);
                }
                b'r' => {
                    let value = match input.get(i)? {
                        b'H' => TaskPriority::H,
                        b'M' => TaskPriority::M,
                        b'L' => TaskPriority::L,
                        _ => return None,
                    };
                    i += 1;
                    priority = Some(value);
                }
                _ => return None,
            }
        }

        Some(Task {
            uuid,
            title: title.to_string(),
            status: TaskStatus::Pending,
            active,
            modified,
            due,
            project,
            tags,
            annotations: Vec::default(),
            priority,
            wait,
            depends,
            uda: HashMap::default(),
        })
    }
}
