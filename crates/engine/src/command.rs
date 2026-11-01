use std::sync::Arc;

use crate::{Engine, Error, Result};
use async_trait::async_trait;

pub(crate) mod registry;

#[derive(Debug, Clone)]
pub struct CommandDescription {
    pub name: Box<str>,
}

#[async_trait]
pub trait CommandHandler: std::fmt::Debug + Send + Sync + 'static {
    /// Handle a command with the given context and arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the command could not be handled for any reason.
    async fn handle(
        &self,
        context: Arc<Engine>,
        args: crate::api::Value,
    ) -> Result<crate::api::Value>;
}

#[async_trait]
pub trait TypedCommandHandler: std::fmt::Debug + Send + Sync + 'static {
    type Method: crate::api::Method;

    /// Handle a command with the given context and typed arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the command could not be handled for any reason.
    async fn handle(
        &self,
        context: Arc<Engine>,
        args: Self::Method,
    ) -> Result<<<Self as TypedCommandHandler>::Method as crate::api::Method>::Result>;
}

#[async_trait]
impl<T: TypedCommandHandler> CommandHandler for T {
    async fn handle(
        &self,
        context: Arc<Engine>,
        args: crate::api::Value,
    ) -> Result<crate::api::Value> {
        let params = args.to_type::<T::Method>().map_err(Error::Other)?;
        let result = self.handle(context, params).await?;
        let result_value = crate::api::Value::from_type(&result);
        Ok(result_value)
    }
}
