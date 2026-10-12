use stride_core::state::KnownPaths;
use stride_plugin_manager::PluginManager;

use crate::{
    BackendHandler, BackendRegistry, CommandHandler, CommandRegistry, NoopNotifier, Notifier,
    Result,
    method::{
        BackendGetHandler, BackendListHandler, PluginDisableHandler, PluginImportHandler,
        PluginRemoveHandler, PluginToggleHandler, RepositoryBackendAddHandler,
        RepositoryBackendGetHandler, RepositoryBackendListHandler, RepositoryBackendRemoveHandler,
        RepositoryBackendSetHandler, RepositoryBackendToggleHandler, RepositoryProjectListHandler,
        RepositoryRemoveHandler, RepositorySyncHandler, RepositoryTagListHandler,
        RepositoryTaskAddHandler, RepositoryTaskQueryHandler, RepositoryTaskRemoveHandler,
        RepositoryTaskUpdateHandler,
    },
};

use super::Engine;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Debug)]
pub struct EngineBuilder {
    known_paths: KnownPaths,
    notifier: Option<Box<dyn Notifier>>,
    commands: CommandRegistry,
    backends: BackendRegistry,
    plugin_manager: Mutex<PluginManager>,
}

impl EngineBuilder {
    #[must_use]
    pub fn new(known_paths: KnownPaths) -> Result<Self> {
        Ok(Self {
            plugin_manager: Mutex::new(PluginManager::new(
                &known_paths.support.as_path().join("plugins"),
            )?),
            known_paths,
            notifier: None,
            commands: CommandRegistry::default(),
            backends: BackendRegistry::default(),
        })
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

    #[must_use]
    pub fn insert_default_methods(self) -> Self {
        self.command("backend.list", BackendListHandler)
            .command("backend.get", BackendGetHandler)
            .command("plugin.import", PluginImportHandler)
            .command("plugin.remove", PluginRemoveHandler)
            .command("plugin.disable", PluginDisableHandler)
            .command("plugin.toggle", PluginToggleHandler)
            .command("repository.backend.add", RepositoryBackendAddHandler)
            .command("repository.backend.list", RepositoryBackendListHandler)
            .command("repository.backend.remove", RepositoryBackendRemoveHandler)
            .command("repository.backend.toggle", RepositoryBackendToggleHandler)
            .command("repository.backend.get", RepositoryBackendGetHandler)
            .command("repository.backend.set", RepositoryBackendSetHandler)
            .command("repository.remove", RepositoryRemoveHandler)
            .command("repository.sync", RepositorySyncHandler)
            .command("repository.tag.list", RepositoryTagListHandler)
            .command("repository.project.list", RepositoryProjectListHandler)
            .command("repository.task.add", RepositoryTaskAddHandler)
            .command("repository.task.update", RepositoryTaskUpdateHandler)
            .command("repository.task.remove", RepositoryTaskRemoveHandler)
            .command("repository.task.query", RepositoryTaskQueryHandler)
    }

    #[must_use]
    pub fn build(self) -> Arc<Engine> {
        Arc::new(Engine {
            known_paths: self.known_paths,
            notifier: self.notifier.unwrap_or_else(|| Box::new(NoopNotifier)),
            commands: self.commands,
            backends: self.backends,
            repositories: Mutex::new(super::Cache::new(Duration::from_secs(60))),
            plugin_manager: self.plugin_manager,
        })
    }
}
