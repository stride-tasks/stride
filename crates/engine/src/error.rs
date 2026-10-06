use stride_api as api;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("api handle not found: {method}")]
    HandlerNotFound { method: Box<str> },
    #[error("api handle failed: {method}:{params} :: {cause}")]
    HandlerFailed {
        method: Box<str>,
        params: api::Value,
        #[source]
        cause: Box<Error>,
    },
    #[error("database error: {0}")]
    Database(
        #[from]
        #[source]
        stride_database::Error,
    ),

    #[error("config error: {0}")]
    Config(
        #[from]
        #[source]
        stride_core::backend::Error,
    ),

    #[error("unknown backend: {name}")]
    UnknownBackend { name: Box<str> },

    #[error("api I/O error: {0}")]
    Io(
        #[from]
        #[source]
        std::io::Error,
    ),
    #[error("api error: {0}")]
    Other(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl<T: std::error::Error + Send + Sync + 'static> From<Box<T>> for Error {
    fn from(value: Box<T>) -> Self {
        Self::Other(value)
    }
}
