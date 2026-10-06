use std::sync::Arc;

use stride_api as api;

use crate::{CommandDescription, CommandRegistry, EngineBuilder, Error, Notifier, Result};

pub(super) mod builder;

#[derive(Debug)]
pub struct Engine {
    notifier: Box<dyn Notifier>,
    commands: CommandRegistry,
}

impl Engine {
    #[must_use]
    pub fn new(notifier: Box<dyn Notifier>) -> Arc<Self> {
        Arc::new(Self {
            notifier,
            commands: CommandRegistry::default(),
        })
    }
}

impl Engine {
    /// Creates a new [`EngineBuilder`] for constructing an [`Engine`].
    #[must_use]
    pub fn builder() -> EngineBuilder {
        EngineBuilder::new()
    }

    /// Notify user of an event with the given notification.
    ///
    /// # Errors
    ///
    /// Returns an error if the notification could not be sent for any reason.
    pub fn notify(self: Arc<Self>, notification: Box<dyn Notification>) -> Result<()> {
        self.clone().notifier.notify(self, notification)
    }

    /// Get the descriptions of all available methods for this context.
    pub fn method_descriptions(self: Arc<Self>) -> Vec<CommandDescription> {
        self.commands
            .iter()
            .map(|(method, _handler)| CommandDescription {
                name: method.into(),
            })
            .collect()
    }

    /// Execute a command with the given method and arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the command could not be executed for any reason.
    pub fn execute_erased(self: Arc<Self>, method: &str, args: api::Value) -> Result<api::Value> {
        let handler = self
            .commands
            .get(method)
            .ok_or_else(|| Error::HandlerNotFound {
                method: method.into(),
            })?;
        let result =
            handler
                .handle(self.clone(), args.clone())
                .map_err(|err| Error::HandlerFailed {
                    method: method.into(),
                    params: args,
                    cause: Box::new(err),
                })?;
        Ok(result)
    }

    /// Execute a command with the given method and arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the method could not be executed for any reason.
    pub fn execute<T: api::Method>(self: Arc<Self>, method: T) -> Result<T::Result> {
        let method_name = T::NAME;

        let params = api::Value::from_type(method);

        let result_value = self.execute_erased(method_name, params)?;
        let result = api::Value::to_type::<T::Result>(&result_value).map_err(Error::Other)?;
        Ok(result)
    }
}

pub trait Notification: std::fmt::Debug + std::any::Any + 'static {
    fn name(&self) -> &'static str;

    fn to_value(&self) -> api::Value;

    /// Convert a `Value` into this notification type.
    ///
    /// # Errors
    ///
    /// Returns an error if the value could not be converted into this notification type.
    fn from_value(value: api::Value) -> Result<Self>
    where
        Self: Sized;
}

impl<T: std::fmt::Debug + api::Notification + 'static> Notification for T {
    fn name(&self) -> &'static str {
        T::NAME
    }

    fn to_value(&self) -> api::Value {
        api::Value::from_type(self)
    }

    fn from_value(value: api::Value) -> Result<Self>
    where
        Self: Sized,
    {
        let value = api::Value::to_type::<T>(&value).map_err(Error::Other)?;
        Ok(value)
    }
}

pub trait Prompt: std::fmt::Debug + std::any::Any + 'static {
    fn target(&self) -> Box<str>;
    fn inputs(&self) -> api::Value {
        api::Value::Map(std::collections::HashMap::default())
    }

    fn summary(&self) -> Box<str>;
    fn description(&self) -> Option<Box<str>> {
        None
    }
}

#[derive(Debug)]
pub struct PromptNotification<T: Prompt> {
    prompt: T,
}

impl<T: Prompt> PromptNotification<T> {
    #[must_use]
    pub fn new(prompt: T) -> Self {
        Self { prompt }
    }
}

impl<T: Prompt> Notification for PromptNotification<T> {
    fn name(&self) -> &'static str {
        <api::UserPromptNotification as api::Notification>::NAME
    }

    fn to_value(&self) -> api::Value {
        let inputs = self.prompt.inputs();

        let prompt = api::UserPromptNotification {
            summary: self.prompt.summary(),
            target: api::UserPromptTarget {
                params: inputs,
                method: self.prompt.target(),
            },
            description: self.prompt.description(),
            actions: Vec::new(),
        };

        api::Value::from_type(&prompt)
    }

    fn from_value(value: api::Value) -> Result<Self>
    where
        Self: Sized,
    {
        Err(Error::HandlerFailed {
            method: <api::UserPromptNotification as api::Notification>::NAME.into(),
            params: value,
            cause: Box::new(Error::HandlerNotFound {
                method: <api::UserPromptNotification as api::Notification>::NAME.into(),
            }),
        })
    }
}
