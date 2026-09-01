use stride_api::{CommandHandler, CommandRegistry, NoopNotifier, Notifier};
use stride_backend::{BackendHandler, registry::BackendRegistry};

use super::Engine;

use std::sync::Arc;

#[derive(Debug, Default)]
pub struct EngineBuilder {
    notifier: Option<Box<dyn Notifier>>,
    commands: CommandRegistry,
    backends: BackendRegistry,
}

impl EngineBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn notifier(mut self, notifier: Box<dyn Notifier>) -> Self {
        self.notifier = Some(notifier);
        self
    }

    #[must_use]
    pub fn command<N, H>(mut self, name: N, handler: H) -> Self
    where
        N: Into<Box<str>>,
        H: CommandHandler + 'static,
    {
        self.commands.insert(name.into(), Box::new(handler));
        self
    }

    #[must_use]
    pub fn backend<T>(mut self, handler: T) -> Self
    where
        T: Into<Box<dyn BackendHandler>>,
    {
        self.backends.insert(handler);
        self
    }

    #[must_use]
    pub fn build(self) -> Arc<Engine> {
        Arc::new(Engine {
            notifier: self.notifier.unwrap_or_else(|| Box::new(NoopNotifier)),
            commands: self.commands,
            backends: self.backends,
        })
    }
}
