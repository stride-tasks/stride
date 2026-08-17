use crate::{Context, Notification, Result};

use std::fmt::Debug;
use std::sync::Arc;

pub trait Notifier: Debug + Send + Sync + 'static {
    /// Notify user of an event with the given context and notification.
    ///
    /// # Errors
    /// Returns an error if the notification could not be sent for any reason.
    fn notify(&self, engine: Arc<dyn Context>, notification: Notification) -> Result<()>;
}

/// A no-op notifier that does nothing when notified.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoopNotifier;

impl Notifier for NoopNotifier {
    fn notify(&self, _: Arc<dyn Context>, _: Notification) -> Result<()> {
        Ok(())
    }
}
