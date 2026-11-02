use async_trait::async_trait;

use crate::Engine;
use crate::engine::Notification;

use std::fmt::Debug;
use std::sync::Arc;

#[async_trait]
pub trait Notifier: Debug + Send + Sync + 'static {
    /// Notify user of an event with the given context and notification.
    ///
    /// # Errors
    /// Returns an error if the notification could not be sent for any reason.
    async fn notify(&self, engine: &Arc<Engine>, notification: Box<dyn Notification>);
}

/// A no-op notifier that does nothing when notified.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoopNotifier;

#[async_trait]
impl Notifier for NoopNotifier {
    async fn notify(&self, _: &Arc<Engine>, _: Box<dyn Notification>) {}
}
