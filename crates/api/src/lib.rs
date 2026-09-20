//! Stride's api implementations.

use std::{any::Any, sync::Arc};

include!(concat!(env!("OUT_DIR"), "/stride_schema.rs"));

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Value {
    Number(f64),
    String(Box<str>),
    Bool(bool),
    Array(Vec<Value>),
    Map(std::collections::HashMap<Box<str>, Value>),
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

pub trait Method: Sized {
    const NAME: &'static str;

    type Result;
}

pub trait Notification: std::fmt::Debug + Any + 'static {
    fn name(&self) -> &'static str;

    fn to_value(&self) -> Value;
    fn from_value(value: Value) -> Result<Self>
    where
        Self: Sized;
}

pub trait Prompt: std::fmt::Debug + Any + 'static {
    fn target(&self) -> Box<str>;
    fn inputs(&self) -> Value {
        Value::Map(std::collections::HashMap::default())
    }

    fn summary(&self) -> Box<str>;
    fn description(&self) -> Option<Box<str>> {
        None
    }
}

impl<T: Prompt> Notification for T {
    fn name(&self) -> &'static str {
        UserPromptNotification::NAME
    }

    fn to_value(&self) -> Value {
        let mut map = std::collections::HashMap::new();
        map.insert("target".into(), Value::String(self.target()));
        map.insert("summary".into(), Value::String(self.summary()));
        if let Some(description) = self.description() {
            map.insert("description".into(), Value::String(description));
        }
        map.insert("inputs".into(), self.inputs());
        Value::Map(map)
    }

    fn from_value(value: Value) -> Result<Self>
    where
        Self: Sized,
    {
        Err(Error::HandlerFailed {
            method: UserPromptNotification::NAME.into(),
            params: value,
            cause: Box::new(Error::HandlerNotFound {
                method: UserPromptNotification::NAME.into(),
            }),
        })
    }
}

mod command;
mod error;
mod notifier;

pub use command::{CommandDescription, CommandHandler, registry::CommandRegistry};
pub use error::{Error, Result};
pub use notifier::{NoopNotifier, Notifier};

pub trait Context: Send + Sync + 'static {
    /// Notify user of an event with the given notification.
    ///
    /// # Errors
    ///
    /// Returns an error if the notification could not be sent for any reason.
    fn notify(self: Arc<Self>, notification: Box<dyn Notification>) -> Result<()>;

    /// Get the descriptions of all available methods for this context.
    fn method_descriptions(self: Arc<Self>) -> Vec<CommandDescription>;

    /// Execute a command with the given method and arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the command could not be executed for any reason.
    fn execute(self: Arc<Self>, method: &str, args: Value) -> Result<Value>;
}
