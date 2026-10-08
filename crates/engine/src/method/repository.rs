use std::sync::Arc;

use stride_core::backend::{BackendRecord, Config};
use uuid::Uuid;

use crate::{Engine, Result, TypedCommandHandler};

#[derive(Debug, Clone, Copy)]
pub struct RepositoryBackendAddHandler;

impl TypedCommandHandler for RepositoryBackendAddHandler {
    type Method = crate::api::RepositoryBackendAddMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryBackendAddMethodResult> {
        let handler = engine.backends().get_or_error(&method.backend)?;
        let name = handler.name();
        let backend_id = Uuid::now_v7();

        let repository = engine.open_repository(method.repository_id)?;
        repository.lock_database().add_backend(&BackendRecord {
            id: backend_id,
            name,
            enabled: false,
            config: Config::default(),
        })?;

        Ok(crate::api::RepositoryBackendAddMethodResult { backend_id })
    }
}

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
