use std::sync::Arc;

use stride_api as api;
use uuid::Uuid;

use crate::api::repository::Repository;

#[derive(Debug, Clone, Copy)]
pub struct RepositorySyncHandler;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RepositorySpec {
    pub(crate) id: Uuid,
}

impl api::CommandHandler for RepositorySyncHandler {
    fn handle(&self, context: Arc<dyn api::Context>, args: api::Value) -> api::Result<Box<str>> {
        let args = serde_json::to_string(&args).map_err(Box::new)?;
        let spec: RepositorySpec = serde_json::from_str(&args).map_err(Box::new)?;

        let mut repository = Repository::open(spec.id).map_err(Box::new)?;
        let changes = repository.sync(&context).map_err(Box::new)?;

        let notification = api::RepositoryChangedNotification {
            repository_id: spec.id,
            changes,
        };

        context.notify(api::Notification::RepositoryChanged(notification))?;
        Ok("".into())
    }
}
