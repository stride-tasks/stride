use std::{any::Any, collections::HashMap};

use uuid::Uuid;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FieldChange {
    #[serde(rename = "type")]
    pub typ: Box<str>,
    pub current: Option<Box<str>>,
    pub previous: Option<Box<str>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TaskChange {
    pub task_id: Uuid,
    pub title: Box<str>,
    pub fields: Vec<FieldChange>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RepositoryChangedNotification {
    pub repository_id: Uuid,
    pub changes: Vec<TaskChange>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Value {
    Number(f64),
    String(Box<str>),
    Bool(bool),
    Array(Vec<Value>),
    Map(HashMap<Box<str>, Value>),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::String(value) => std::fmt::Debug::fmt(value, f),
            Self::Bool(value) => value.fmt(f),
            Self::Array(array) => {
                f.write_str("[")?;
                for (i, value) in array.iter().enumerate() {
                    value.fmt(f)?;
                    if i + 1 != array.len() {
                        f.write_str(", ")?;
                    }
                }
                f.write_str("]")
            }
            Self::Map(map) => {
                f.write_str("{")?;
                for (i, (key, value)) in map.iter().enumerate() {
                    std::fmt::Debug::fmt(key, f)?;
                    f.write_str(": ")?;
                    value.fmt(f)?;
                    if i + 1 != map.len() {
                        f.write_str(", ")?;
                    }
                }
                f.write_str("}")
            }
            Self::Null => f.write_str("null"),
        }
    }
}

pub const PROMPT_METHOD: &str = "stride.notification.prompt";

pub trait Prompt: std::fmt::Debug + Any + 'static {
    fn target(&self) -> Box<str>;
    fn inputs(&self) -> Value {
        Value::Map(HashMap::default())
    }

    fn summary(&self) -> Box<str>;
    fn description(&self) -> Option<Box<str>> {
        None
    }
}

#[derive(Debug)]
pub enum Notification {
    Prompt(Box<dyn Prompt>),
    RepositoryChanged(RepositoryChangedNotification),
}
