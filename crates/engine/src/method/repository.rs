use std::sync::Arc;

use crate::{Engine, Result, TypedCommandHandler};

#[derive(Debug, Clone, Copy)]
pub struct RepositorySyncHandler;

impl TypedCommandHandler for RepositorySyncHandler {
    type Method = crate::api::RepositorySyncMethod;

    fn handle(
        &self,
        context: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositorySyncMethodResult> {
        let repository = context.open_repository(method.id)?;
        let changes = repository.sync(&context).map_err(Box::new)?;

        context.notify(Box::new(crate::api::RepositoryChangedNotification {
            repository_id: method.id,
            changes,
        }))?;
        Ok(crate::api::RepositorySyncMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryTagListHandler;

impl TypedCommandHandler for RepositoryTagListHandler {
    type Method = crate::api::RepositoryTagListMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryTagListMethodResult> {
        let repository = engine.open_repository(method.id)?;
        let tags = repository.lock_database().used_tags()?;
        Ok(crate::api::RepositoryTagListMethodResult { tags })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryProjectListHandler;

impl TypedCommandHandler for RepositoryProjectListHandler {
    type Method = crate::api::RepositoryProjectListMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryProjectListMethodResult> {
        let repository = engine.open_repository(method.id)?;
        let projects = repository.lock_database().used_projects()?;

        Ok(crate::api::RepositoryProjectListMethodResult { projects })
    }
}
