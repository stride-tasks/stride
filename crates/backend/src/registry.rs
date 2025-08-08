use std::collections::HashMap;

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
}
