use std::str::Utf8Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveVersionedKind {
    Annotation,
    Uda,
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum BlobError {
    #[error("unknown version {version} in {kind:?}")]
    UnknownVersion {
        version: u8,
        kind: PrimitiveVersionedKind,
    },
    #[error("missing length")]
    MissingLength,
    #[error("invalid UTF8: {0}")]
    InvalidUt8(#[from] Utf8Error),
    #[error("invalid timestamp")]
    InvalidTimestamp,
    #[error("abrupt end")]
    AbruptEnd,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("blob error: {0}")]
    Blob(#[from] BlobError),
}
