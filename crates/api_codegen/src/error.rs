pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error in {path}: {source}")]
    Json {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("duplicate schema $id: {id}")]
    DuplicateSchemaId { id: String },
    #[error("schema references unknown $id: {reference}")]
    UnknownReference { reference: String },
}
