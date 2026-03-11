use uuid::Uuid;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("crdt error: {0}")]
    Crdt(#[from] stride_crdt::Error),
    #[error("blob error: {0}")]
    Serialize(#[from] stride_serialize::Error),
    #[error("task not found: {id}")]
    TaskNotFound { id: Uuid },
}
