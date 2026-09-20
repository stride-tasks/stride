use std::sync::Arc;

use stride_api as api;

use crate::EngineBuilder;

pub(super) mod builder;

#[derive(Debug)]
pub struct Engine {
    notifier: Box<dyn api::Notifier>,
    commands: api::CommandRegistry,
}

impl Engine {
    #[must_use]
    pub fn new(notifier: Box<dyn api::Notifier>) -> Arc<Self> {
        Arc::new(Self {
            notifier,
            commands: api::CommandRegistry::default(),
        })
    }
}

impl api::Context for Engine {
    fn notify(self: Arc<Self>, notification: Box<dyn api::Notification>) -> api::Result<()> {
        self.clone().notifier.notify(self, notification)
    }

    fn method_descriptions(self: Arc<Self>) -> Vec<api::CommandDescription> {
        self.commands
            .iter()
            .map(|(method, _handler)| api::CommandDescription {
                name: method.into(),
            })
            .collect()
    }

    fn execute(self: Arc<Self>, method: &str, args: api::Value) -> api::Result<api::Value> {
        let handler = self
            .commands
            .get(method)
            .ok_or_else(|| api::Error::HandlerNotFound {
                method: method.into(),
            })?;
        let result = handler.handle(self.clone(), args.clone()).map_err(|err| {
            api::Error::HandlerFailed {
                method: method.into(),
                params: args,
                cause: Box::new(err),
            }
        })?;
        Ok(result)
    }
}

impl Engine {
    /// Creates a new [`EngineBuilder`] for constructing an [`Engine`].
    #[must_use]
    pub fn builder() -> EngineBuilder {
        EngineBuilder::new()
    }
}
