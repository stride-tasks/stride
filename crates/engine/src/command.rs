use std::sync::Arc;

use crate::{Engine, Error, Result};

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
    fn handle(&self, context: Arc<Engine>, args: crate::api::Value) -> Result<crate::api::Value>;
}

pub trait TypedCommandHandler: std::fmt::Debug + Send + Sync + 'static {
    type Method: crate::api::Method;

    /// Handle a command with the given context and typed arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the command could not be handled for any reason.
    fn handle(
        &self,
        context: Arc<Engine>,
        args: Self::Method,
    ) -> Result<<<Self as TypedCommandHandler>::Method as crate::api::Method>::Result>;
}

impl<T: TypedCommandHandler> CommandHandler for T {
    fn handle(&self, context: Arc<Engine>, args: crate::api::Value) -> Result<crate::api::Value> {
        let params = args.to_type::<T::Method>().map_err(Error::Other)?;
        let result = self.handle(context, params)?;
        let result_value = crate::api::Value::from_type(&result);
        Ok(result_value)
    }
}
