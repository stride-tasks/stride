#![allow(clippy::doc_markdown)]

use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub mod annotation;

pub type Date = DateTime<Utc>;

pub use annotation::Annotation;
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
    #[allow(clippy::missing_panics_doc)]
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
    #[must_use]
    pub fn to_data(&self) -> Vec<u8> {
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
        if let Some(project) = &self.project {
            result.push(b'p');
            result.extend_from_slice(&(project.len() as u32).to_be_bytes());
            result.extend_from_slice(project.as_bytes());
        }
        if let Some(priority) = self.priority {
            result.push(b'r');
            result.push(priority.as_str().as_bytes()[0]);
        }
        for tag in &self.tags {
            result.push(b't');
            result.extend_from_slice(&(tag.len() as u32).to_be_bytes());
            result.extend_from_slice(tag.as_bytes());
        }
        for depend in &self.depends {
            result.push(b'n');
            result.extend_from_slice(depend.as_bytes());
        }
        // TODO: Consider adding external annoations (separate files for annotations).
        if !self.annotations.is_empty() {
            result.push(b'a');
            result.extend_from_slice(&(self.annotations.len() as u32).to_be_bytes());
            for annotation in &self.annotations {
                result.extend(&annotation.entry.timestamp_micros().to_be_bytes());
                result.extend_from_slice(&(annotation.description.len() as u32).to_be_bytes());
                result.extend_from_slice(annotation.description.as_bytes());
            }
        }
        if !self.uda.is_empty() {
            todo!("UDA not implemented")
        }
        result
    }

    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn from_data(input: &[u8]) -> Option<Task> {
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
        let mut annotations = Vec::new();
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
                    let len =
                        u32::from_be_bytes(input.get(i..i + size_of::<u32>())?.try_into().ok()?)
                            as usize;
                    i += size_of::<u32>();
                    let bytes = input.get(i..i + len)?;
                    let value = std::str::from_utf8(bytes).ok()?;
                    i += len;
                    tags.push(value.into());
                }
                b'p' => {
                    let len =
                        u32::from_be_bytes(input.get(i..i + size_of::<u32>())?.try_into().ok()?)
                            as usize;
                    i += size_of::<u32>();
                    let bytes = input.get(i..i + len)?;
                    let value = std::str::from_utf8(bytes).ok()?;
                    i += len;
                    project = Some(value.into());
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
                b'a' => {
                    let len =
                        u32::from_be_bytes(input.get(i..i + size_of::<u32>())?.try_into().ok()?);
                    i += size_of::<u32>();

                    annotations = Vec::with_capacity(len as usize);

                    for _ in 0..len {
                        let timestamp = input.get(i..(i + size_of::<i64>()))?;
                        let timestamp = i64::from_be_bytes(timestamp.try_into().ok()?);
                        let datetime = DateTime::from_timestamp_micros(timestamp)?;
                        i += size_of::<i64>();

                        let title_len = u32::from_be_bytes(
                            input.get(i..i + size_of::<u32>())?.try_into().ok()?,
                        ) as usize;
                        i += size_of::<u32>();

                        let title_bytes = input.get(i..i + title_len)?;
                        i += title_len;

                        let title = std::str::from_utf8(title_bytes).ok()?;

                        annotations.push(Annotation {
                            entry: datetime,
                            description: title.to_string(),
                        });
                    }
                }
                _ => return None,
            }
        }

        Some(Task {
            uuid,
            title: title.into(),
            status: TaskStatus::Pending,
            active,
            modified,
            due,
            project,
            tags,
            annotations,
            priority,
            wait,
            depends,
            uda: HashMap::default(),
        })
    }

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
            status: v.get_status().into(),
            title: v.get_description().into(),
            active: v.get_status() == taskchampion::Status::Pending,
            modified: v.get_modified(),
            due: v.get_due(),
            project: v.get_value("project").map(Into::into),
            tags: vec![],
            annotations: v.get_annotations().map(Into::into).collect(),
            priority: taskchampion_priority_to_task_status(v.get_priority()),
            wait: v.get_wait(),
            depends: v.get_dependencies().collect(),
            // TODO: Remove use of deprecated function.
            #[allow(deprecated)]
            uda: v
                .get_udas()
                .map(|((namespace, key), value)| (format!("{namespace}.{key}"), value.to_owned()))
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
