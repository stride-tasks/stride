use stride_plugin_manager::PluginManager;

use crate::{
    BackendHandler, BackendRegistry, CommandHandler, CommandRegistry, KnownPaths, NoopNotifier,
    Notifier, Result,
    method::{
        BackendGetHandler, BackendListHandler, LogEventHandler, LogGetHandler,
        PluginDisableHandler, PluginImportHandler, PluginListHandler, PluginRemoveHandler,
        PluginToggleHandler, PluginValidateHandler, RepositoryAddHandler,
        RepositoryBackendAddHandler, RepositoryBackendGetHandler, RepositoryBackendListHandler,
        RepositoryBackendRemoveHandler, RepositoryBackendSetHandler,
        RepositoryBackendToggleHandler, RepositoryListHandler, RepositoryProjectListHandler,
        RepositoryRemoveHandler, RepositorySyncHandler, RepositoryTagListHandler,
        RepositoryTaskAddHandler, RepositoryTaskQueryHandler, RepositoryTaskRemoveHandler,
        RepositoryTaskUpdateHandler, RepositoryUpdateHandler, SettingGetHandler,
    },
};

use super::Engine;

use std::sync::Arc;

#[derive(Debug)]
pub struct EngineBuilder {
    known_paths: KnownPaths,
    notifier: Option<Box<dyn Notifier>>,
    commands: CommandRegistry,
    backends: BackendRegistry,
    plugin_manager: PluginManager,
}

impl EngineBuilder {
    /// # Errors
    ///
    /// Returns an error if the plugin directory cannot be loaded.
    pub fn new(known_paths: KnownPaths) -> Result<Self> {
        let mut plugin_manager =
            PluginManager::new(&known_paths.support.as_path().join("plugins"))?;
        plugin_manager.load()?;

        Ok(Self {
            plugin_manager,
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
            .command("log.event", LogEventHandler)
            .command("log.get", LogGetHandler)
            .command("plugin.list", PluginListHandler)
            .command("plugin.validate", PluginValidateHandler)
            .command("plugin.import", PluginImportHandler)
            .command("plugin.remove", PluginRemoveHandler)
            .command("plugin.disable", PluginDisableHandler)
            .command("plugin.toggle", PluginToggleHandler)
            .command("repository.add", RepositoryAddHandler)
            .command("repository.list", RepositoryListHandler)
            .command("repository.update", RepositoryUpdateHandler)
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
            .command("setting.get", SettingGetHandler)
    }

    #[must_use]
    pub fn build(self) -> Arc<Engine> {
        Engine::with_plugin_manager(
            self.known_paths,
            self.notifier.unwrap_or_else(|| Box::new(NoopNotifier)),
            self.commands,
            self.backends,
            self.plugin_manager,
        )
    }
}
