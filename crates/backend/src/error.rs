use std::{any::Any, fmt::Display};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub trait BackendError: Any + std::error::Error + Display + Sync + Send + 'static {}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    Database(#[from] stride_database::Error),

    #[error("config error: {0}")]
    Config(#[from] stride_core::backend::Error),

    #[error("unknown backend: {name}")]
    UnknownBackend { name: Box<str> },

    #[error("{0}")]
    Other(Box<dyn BackendError>),
}

impl Error {
    #[inline]
    #[must_use]
    pub fn downcast_ref<T: BackendError>(&self) -> Option<&T> {
        let Self::Other(error) = self else {
            return None;
        };

        let any: &dyn Any = error.as_ref();
        any.downcast_ref::<T>()
    }
}

impl From<Box<dyn BackendError>> for Error {
    #[inline]
    fn from(value: Box<dyn BackendError>) -> Self {
        Self::Other(value)
    }
}

impl<T: BackendError> From<T> for Error {
    #[inline]
    fn from(value: T) -> Self {
        Self::Other(Box::new(value))
    }
}
