use std::{collections::HashMap, default};

use chrono::NaiveDateTime;

pub mod annotation;

pub type Date = NaiveDateTime;

pub use annotation::Annotation;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ToBase64;
pub type Tag = String;
pub type Project = String;
pub type Priority = String;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct Task {
    #[builder(default = "Uuid::new_v4()")]
    pub uuid: Uuid,

    #[builder(default = "Date::from(chrono::Utc::now().naive_utc())")]
    pub entry: Date,

    #[builder(default = "TaskStatus::Pending")]
    #[serde(skip)]
    pub status: TaskStatus,

    pub description: String,

    #[builder(default)]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<Date>,

    #[builder(default)]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<Date>,

    #[builder(default)]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,

    #[builder(default)]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,

    #[builder(default)]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<Annotation>,

    #[builder(default)]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,

    #[builder(default)]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<Date>,

    #[builder(default)]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Date>,

    #[builder(default)]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub depends: Vec<Uuid>,

    #[builder(default)]
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub uda: HashMap<String, String>,
}

impl TaskBuilder {
    #[allow(dead_code)]
    pub fn with_description<T: Into<String>>(description: T) -> Self {
        let mut this = Self::default();
        this.description(description);
        this
    }
}

impl Task {
    fn to_data(&self) -> String {
        let mut result = self.uuid.to_base64();
        for c in self.description.chars() {
            if c == '\n' {
                result.push('\\');
                result.push('n');
            } else {
                result.push(c);
            }
        }
        result
    }
}
