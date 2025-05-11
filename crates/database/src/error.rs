pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum AnnotationParseError {
    #[error("unknown version: {version}")]
    UnknownVersion { version: u8 },
    #[error("missing length")]
    MissingLength,
    #[error("missing entry timestamp")]
    MissingEntryTimestamp,
    #[error("missing text")]
    MissingText,
    #[error("invalid UTF8")]
    InvalidUt8,
    #[error("invalid timestamp")]
    InvalidTimestamp,
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum UdaParseError {
    #[error("unknown version: {version}")]
    UnknownVersion { version: u8 },
    #[error("missing length")]
    MissingLength,
    #[error("missing namespace")]
    MissingNamespace,
    #[error("missing key")]
    MissingKey,
    #[error("missing value")]
    MissingValue,
    #[error("invalid UTF8")]
    InvalidUt8,
    #[error("invalid timestamp")]
    InvalidTimestamp,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("annotation parse error: {0}")]
    AnnotationParse(AnnotationParseError),
    #[error("uda parse error: {0}")]
    UdaParse(UdaParseError),
}
