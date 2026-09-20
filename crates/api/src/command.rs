use std::sync::Arc;

use crate::{Context, Result, Value};

pub(crate) mod registry;

#[derive(Debug, Clone)]
pub struct CommandDescription {
    pub name: Box<str>,
}

pub trait CommandHandler: std::fmt::Debug + Send + Sync + 'static {
    /// Handle a command with the given context and arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the command could not be handled for any reason.
    fn handle(&self, context: Arc<dyn Context>, args: Value) -> Result<Value>;
}

pub trait TypedCommandHandler: std::fmt::Debug + Send + Sync + 'static {
    type Method: crate::Method;

    /// Handle a command with the given context and typed arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the command could not be handled for any reason.
    fn handle(
        &self,
        context: Arc<dyn Context>,
        args: Self::Method,
    ) -> Result<<<Self as TypedCommandHandler>::Method as crate::Method>::Result>;
}

impl<T: TypedCommandHandler> CommandHandler for T {
    fn handle(&self, context: Arc<dyn Context>, args: Value) -> Result<Value> {
        let args = serde_json::to_value(&args)?;
        let params = serde_json::from_value::<T::Method>(args)?;
        let result = self.handle(context, params)?;
        let result_value = serde_json::to_value(&result)?;
        let result_value = serde_json::from_value::<Value>(result_value)?;
        Ok(result_value)
    }
}
