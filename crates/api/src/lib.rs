//! Stride's api implementations.

use std::sync::Arc;

mod command;
mod error;
mod notification;
mod notifier;

pub use command::{CommandHandler, registry::CommandRegistry};
pub use error::{Error, Result};
pub use notification::{
    FieldChange, Notification, PROMPT_METHOD, Prompt, RepositoryChangedNotification, TaskChange,
    Value,
};
pub use notifier::{NoopNotifier, Notifier};

pub trait Context: Send + Sync + 'static {
    /// Notify user of an event with the given notification.
    ///
    /// # Errors
    ///
    /// Returns an error if the notification could not be sent for any reason.
    fn notify(self: Arc<Self>, notification: Notification) -> Result<()>;

    /// Execute a command with the given method and arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the command could not be executed for any reason.
    fn execute(self: Arc<Self>, method: &str, args: Value) -> Result<Value>;
}
