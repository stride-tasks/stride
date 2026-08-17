pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
#[error("api error: {inner}")]
pub struct Error {
    #[source]
    inner: Box<dyn std::error::Error + Send + Sync>,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}

impl<T: std::error::Error + Send + Sync + 'static> From<Box<T>> for Error {
    fn from(value: Box<T>) -> Self {
        Self { inner: value }
    }
}
