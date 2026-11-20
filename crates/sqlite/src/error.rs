pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error(
        "invalid storage key {key:?}: must be non-empty '.' separated names made of ASCII alphanumerics, '-' or '_'"
    )]
    InvalidKey { key: Box<str> },
    #[error("invalid storage value for key {key:?}: {source}")]
    InvalidValue {
        key: Box<str>,
        source: stride_api::config::DecodeError,
    },
}
