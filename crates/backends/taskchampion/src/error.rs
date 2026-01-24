use stride_backend::error::BackendError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("taskchampion error: {0}")]
    TaskChampion(#[from] ::taskchampion::Error),
}

impl BackendError for Error {}
