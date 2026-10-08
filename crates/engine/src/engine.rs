use std::{
    sync::{Arc, Mutex, MutexGuard},
    time::{Duration, Instant},
};

use stride_api as api;
use stride_core::state::KnownPaths;
use uuid::Uuid;

use crate::{
    BackendRegistry, CommandDescription, CommandRegistry, EngineBuilder, Error, Notifier,
    Repository, Result, cache::Cache,
};

pub(super) mod builder;

#[derive(Debug)]
pub struct Engine {
    known_paths: KnownPaths,
    notifier: Box<dyn Notifier>,
    commands: CommandRegistry,
    backends: BackendRegistry,
    repositories: Mutex<Cache<Repository>>,
}

impl Engine {
    #[must_use]
    pub fn new(known_paths: KnownPaths, notifier: Box<dyn Notifier>) -> Arc<Self> {
        Arc::new(Self {
            known_paths,
            notifier,
            commands: CommandRegistry::default(),
            backends: BackendRegistry::default(),
            repositories: Mutex::new(Cache::new(Duration::from_secs(60))),
        })
    }
}

impl Engine {
    /// Creates a new [`EngineBuilder`] for constructing an [`Engine`].
    #[must_use]
    pub fn builder(known_paths: KnownPaths) -> EngineBuilder {
        EngineBuilder::new(known_paths)
    }

    /// Notify user of an event with the given notification.
    ///
    /// # Errors
    ///
    /// Returns an error if the notification could not be sent for any reason.
    pub fn notify(self: &Arc<Self>, notification: Box<dyn Notification>) -> Result<()> {
        self.clone().notifier.notify(self, notification)
    }

    pub(crate) fn lock_repositories(self: &Arc<Self>) -> MutexGuard<'_, Cache<Repository>> {
        self.repositories
            .lock()
            .unwrap_or_else(|cache| cache.into_inner())
    }

    /// Open a repository instance and cache it for one minute of inactivity.
    ///
    /// The cache keeps using the same [`Arc`] while a caller still has a strong reference.
    /// Expired entries are evicted in FIFO order once they are no longer referenced.
    pub fn open_repository(self: &Arc<Self>, id: Uuid) -> Result<Arc<Repository>> {
        let mut repositories = self.lock_repositories();
        let now = Instant::now();

        if let Some(entry) = repositories.get_mut(id) {
            entry.last_used = now;
            let repository = entry.value.clone();

            repositories.evict_expired(now);
            return Ok(repository);
        }

        repositories.evict_expired(now);

        let repository = Repository::open(id, self.known_paths())?;
        let repository = Arc::new(repository);

        repositories.insert(id, repository.clone());
        Ok(repository)
    }

    /// Remove a repository and its associated data from the engine and the filesystem.
    ///
    /// # Errors
    ///
    /// Returns an error if the repository could not be removed for any reason.
    pub fn remove_repository(self: &Arc<Self>, id: Uuid) -> Result<()> {
        let mut repositories = self.lock_repositories();
        repositories.remove(id);

        let root_path = self.known_paths().repository_path(id);
        std::fs::remove_dir_all(&root_path)?;
        Ok(())
    }

    /// Execute a command with the given method and arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the command could not be executed for any reason.
    pub fn execute_erased(self: &Arc<Self>, method: &str, args: api::Value) -> Result<api::Value> {
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
    pub fn execute<T: api::Method>(self: &Arc<Self>, method: T) -> Result<T::Result> {
        let method_name = T::NAME;

        let params = api::Value::from_type(method);

        let result_value = self.execute_erased(method_name, params)?;
        let result = api::Value::to_type::<T::Result>(&result_value).map_err(Error::Other)?;
        Ok(result)
    }

    /// Get the known paths for this engine instance.
    pub fn known_paths(self: &Arc<Self>) -> &KnownPaths {
        &self.known_paths
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

    pub fn backends(self: &Arc<Self>) -> &BackendRegistry {
        &self.backends
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
