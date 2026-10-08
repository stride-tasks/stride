use stride_core::state::KnownPaths;

use crate::{
    BackendHandler, BackendRegistry, CommandHandler, CommandRegistry, NoopNotifier, Notifier,
    method::{
        BackendListHandler, RepositoryBackendAddHandler, RepositoryBackendListHandler,
        RepositoryBackendRemoveHandler, RepositoryBackendToggleHandler,
        RepositoryBackendUpdateHandler, RepositoryProjectListHandler, RepositorySyncHandler,
        RepositoryTagListHandler,
    },
};

use super::Engine;

use std::{sync::Arc, time::Duration};

#[derive(Debug)]
pub struct EngineBuilder {
    known_paths: KnownPaths,
    notifier: Option<Box<dyn Notifier>>,
    commands: CommandRegistry,
    backends: BackendRegistry,
}

impl EngineBuilder {
    #[must_use]
    pub fn new(known_paths: KnownPaths) -> Self {
        Self {
            known_paths,
            notifier: None,
            commands: CommandRegistry::default(),
            backends: BackendRegistry::default(),
        }
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
    pub fn backend<T>(mut self, backend: T) -> Self
    where
        T: Into<Box<dyn BackendHandler + 'static>>,
    {
        self.backends.insert(backend.into());
        self
    }

    pub fn insert_default_methods(self) -> Self {
        self.command("backend.list", BackendListHandler)
            .command("repository.backend.add", RepositoryBackendAddHandler)
            .command("repository.backend.list", RepositoryBackendListHandler)
            .command("repository.backend.remove", RepositoryBackendRemoveHandler)
            .command("repository.backend.toggle", RepositoryBackendToggleHandler)
            .command("repository.backend.update", RepositoryBackendUpdateHandler)
            .command("repository.sync", RepositorySyncHandler)
            .command("repository.tag.list", RepositoryTagListHandler)
            .command("repository.project.list", RepositoryProjectListHandler)
    }

    #[must_use]
    pub fn build(self) -> Arc<Engine> {
        Arc::new(Engine {
            known_paths: self.known_paths,
            notifier: self.notifier.unwrap_or_else(|| Box::new(NoopNotifier)),
            commands: self.commands,
            backends: self.backends,
            repositories: std::sync::Mutex::new(super::Cache::new(Duration::from_secs(60))),
        })
    }
}
