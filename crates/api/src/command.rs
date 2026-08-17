use std::sync::Arc;

use crate::{Context, Result};

pub(crate) mod registry;

pub trait CommandHandler: std::fmt::Debug + Send + Sync + 'static {
    /// Handle a command with the given context and arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the command could not be handled for any reason.
    fn handle(&self, context: Arc<dyn Context>, args: &str) -> Result<Box<str>>;
}
