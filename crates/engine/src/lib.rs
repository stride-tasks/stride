//! Stride's Engine.

mod backend;
mod command;
mod engine;
mod error;
mod notifier;

pub use backend::{Backend, BackendHandler, registry::BackendRegistry};
pub use command::{
    CommandDescription, CommandHandler, TypedCommandHandler, registry::CommandRegistry,
};
pub use engine::{Engine, Notification, Prompt, PromptNotification, builder::EngineBuilder};
pub use error::{Error, Result};
pub use notifier::{NoopNotifier, Notifier};

/// Re-export the stride_api crate for convenience.
pub use stride_api as api;

// impl Notification for RepositoryChangedNotification {
//     fn name(&self) -> &'static str {
//         "repository.changed"
//     }
//     fn to_value(&self) -> Value {
//         let value = serde_json::to_value(self).expect("Failed to serialize to JSON");
//         serde_json::from_value(value).expect("Failed to deserialize from JSON")
//     }
//     fn from_value(value: Value) -> Result<Self>
//     where
//         Self: Sized,
//     {
//         let value = serde_json::to_value(value)?;
//         Ok(serde_json::from_value(value)?)
//     }
// }
