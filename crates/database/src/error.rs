use std::str::Utf8Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone, Copy)]
pub enum BlobVersionedKind {
    Operation,
    Annotation,
    Uda,
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum BlobError {
    #[error("unknown version {version} in {kind:?}")]
    UnknownVersion {
        version: u8,
        kind: BlobVersionedKind,
    },
    #[error("missing length")]
    MissingLength,
    #[error("invalid UTF8: {0}")]
    InvalidUt8(#[from] Utf8Error),
    #[error("invalid timestamp")]
    InvalidTimestamp,
    #[error("unknown task status kind {kind}")]
    UnknownTaskStatus { kind: u8 },
    #[error("unknown task priority kind {kind}")]
    UnknownTaskPriority { kind: u8 },
    #[error("unknown operation kind {kind}")]
    UnknownOperationKind { kind: u8 },
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
