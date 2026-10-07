//! Stride's Engine.

mod backend;
pub(crate) mod cache;
mod command;
mod engine;
mod error;
mod notifier;
mod repository;

pub mod method;

pub use backend::{Backend, BackendHandler, registry::BackendRegistry};
pub use command::{
    CommandDescription, CommandHandler, TypedCommandHandler, registry::CommandRegistry,
};
pub use engine::{Engine, Notification, Prompt, PromptNotification, builder::EngineBuilder};
pub use error::{Error, Result};
pub use notifier::{NoopNotifier, Notifier};
pub use repository::Repository;

/// Re-export the stride_api crate for convenience.
pub use stride_api as api;
