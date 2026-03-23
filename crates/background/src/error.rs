use std::sync::Arc;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub trait ToBackgroundError: std::any::Any + std::error::Error + Send + Sync + 'static {}

impl<T: ToBackgroundError> From<T> for Error {
    fn from(value: T) -> Self {
        Self::Task(Arc::new(value))
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("background thread is closed")]
    BackgroundThreadClosed,
    #[error("task error: {0}")]
    Task(#[source] Arc<dyn ToBackgroundError>),
}

impl Error {
    #[must_use]
    pub fn as_task_error(&self) -> Option<&Arc<dyn ToBackgroundError>> {
        if let Self::Task(error) = &self {
            return Some(error);
        }
        None
    }

    #[must_use]
    pub fn downcast_ref<T: ToBackgroundError>(&self) -> Option<&T> {
        let task_error = self.as_task_error()?;
        let x: &dyn std::any::Any = task_error.as_ref();
        x.downcast_ref::<T>()
    }
}
