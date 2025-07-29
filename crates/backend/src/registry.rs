use std::collections::HashMap;

use stride_core::state::KnownPaths;
use stride_database::Database;
use uuid::Uuid;

use crate::{BackendHandler, Error};

#[derive(Debug, Default)]
pub struct Registry {
    map: HashMap<Box<str>, Box<dyn BackendHandler>>,
}

impl Registry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T>(&mut self, value: T) -> Option<Box<dyn BackendHandler>>
    where
        T: Into<Box<dyn BackendHandler>>,
    {
        let handler = value.into();
        self.map.insert(handler.name(), handler)
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<&dyn BackendHandler> {
        self.map.get(name).map(Box::as_ref)
    }

    pub fn get_or_error(&self, name: &str) -> Result<&dyn BackendHandler, Error> {
        self.get(name)
            .ok_or_else(|| Error::UnknownBackend { name: name.into() })
    }

    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.map.keys().map(Box::as_ref)
    }

    pub fn sync_all(
        &self,
        repository_id: Uuid,
        database: &mut Database,
        known_paths: &KnownPaths,
    ) -> Result<(), Error> {
        let backends = database.backends()?;
        for mut backend in backends {
            if !backend.enabled {
                continue;
            }

            let Some(handler) = self.get(&backend.name) else {
                continue;
            };

            let config = backend
                .config
                .align(&handler.config_schema())
                .map_err(Error::Config)?;

            if config != backend.config {
                backend.config = config;
                database.update_backend(&backend)?;
            }

            let path = known_paths
                .backend_path(repository_id)
                .join(handler.name().as_ref())
                .join(backend.id.to_string());

            let mut backend = handler.create(&backend.config, &path, known_paths)?;

            backend.sync(database)?;
        }

        Ok(())
    }
}
