use std::{collections::HashMap, sync::Arc};

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
    fn handle(&self, context: Arc<dyn api::Context>, args: api::Value) -> api::Result<api::Value> {
        let args = serde_json::to_string(&args).map_err(Box::new)?;
        let spec: RepositorySpec = serde_json::from_str(&args).map_err(Box::new)?;

        let mut repository = Repository::open(spec.id).map_err(Box::new)?;
        let changes = repository.sync(&context).map_err(Box::new)?;

        let notification = api::RepositoryChangedNotification {
            repository_id: spec.id,
            changes,
        };

        context.notify(api::Notification::RepositoryChanged(notification))?;
        Ok(api::Value::Map(HashMap::new()))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryTagListHandler;

impl api::CommandHandler for RepositoryTagListHandler {
    fn handle(&self, _: Arc<dyn api::Context>, args: api::Value) -> api::Result<api::Value> {
        let args = serde_json::to_string(&args).map_err(Box::new)?;
        let spec: RepositorySpec = serde_json::from_str(&args).map_err(Box::new)?;

        let repository = Repository::open(spec.id).map_err(Box::new)?;
        let tags = repository
            .database()
            .lock()
            .unwrap()
            .used_tags()
            .map_err(Box::new)?;

        let mut result = Vec::new();
        for tag in tags {
            result.push(api::Value::Map(HashMap::from([(
                "id".into(),
                api::Value::String(tag),
            )])));
        }

        Ok(api::Value::Map(HashMap::from([(
            "tags".into(),
            api::Value::Array(result),
        )])))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryProjectsListHandler;

impl api::CommandHandler for RepositoryProjectsListHandler {
    fn handle(&self, _: Arc<dyn api::Context>, args: api::Value) -> api::Result<api::Value> {
        let args = serde_json::to_string(&args).map_err(Box::new)?;
        let spec: RepositorySpec = serde_json::from_str(&args).map_err(Box::new)?;

        let repository = Repository::open(spec.id).map_err(Box::new)?;
        let projects = repository
            .database()
            .lock()
            .unwrap()
            .used_projects()
            .map_err(Box::new)?;

        let mut result = Vec::new();
        for project in projects {
            result.push(api::Value::Map(HashMap::from([(
                "id".into(),
                api::Value::String(project),
            )])));
        }

        Ok(api::Value::Map(HashMap::from([(
            "projects".into(),
            api::Value::Array(result),
        )])))
    }
}
