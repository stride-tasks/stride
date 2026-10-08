use std::sync::Arc;

use stride_core::backend::{BackendRecord, Config};
use uuid::Uuid;

use crate::{Engine, Error, Result, TypedCommandHandler};

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
pub struct RepositoryBackendRemoveHandler;

impl TypedCommandHandler for RepositoryBackendRemoveHandler {
    type Method = crate::api::RepositoryBackendRemoveMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryBackendRemoveMethodResult> {
        let repository = engine.open_repository(method.repository_id)?;
        repository
            .lock_database()
            .delete_backend(method.backend_id)?;

        Ok(crate::api::RepositoryBackendRemoveMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryBackendToggleHandler;

impl TypedCommandHandler for RepositoryBackendToggleHandler {
    type Method = crate::api::RepositoryBackendToggleMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryBackendToggleMethodResult> {
        let repository = engine.open_repository(method.repository_id)?;
        let new_state = repository
            .lock_database()
            .toggle_backend(method.backend_id)?;

        Ok(crate::api::RepositoryBackendToggleMethodResult { state: new_state })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryBackendUpdateHandler;

impl TypedCommandHandler for RepositoryBackendUpdateHandler {
    type Method = crate::api::RepositoryBackendUpdateMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryBackendUpdateMethodResult> {
        let repository = engine.open_repository(method.repository_id)?;
        repository
            .lock_database()
            .update_backend(dbg!(&BackendRecord {
                id: method.backend_instance.id,
                name: method.backend_instance.name,
                enabled: method.backend_instance.state == crate::api::BackendInstanceState::Enabled,
                config: method
                    .backend_instance
                    .configuration
                    .to_type()
                    .map_err(Error::Other)?,
            }))?;

        Ok(crate::api::RepositoryBackendUpdateMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryBackendListHandler;

impl TypedCommandHandler for RepositoryBackendListHandler {
    type Method = crate::api::RepositoryBackendListMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryBackendListMethodResult> {
        let repository = engine.open_repository(method.repository_id)?;
        let backend_instances = repository.lock_database().backends()?;

        Ok(crate::api::RepositoryBackendListMethodResult {
            backends: backend_instances
                .into_iter()
                .map(|backend| crate::api::BackendInstance {
                    name: backend.name,
                    id: backend.id,
                    state: if backend.enabled {
                        crate::api::BackendInstanceState::Enabled
                    } else {
                        crate::api::BackendInstanceState::Disabled
                    },
                    configuration: crate::api::Value::from_type(backend.config),
                })
                .collect(),
        })
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
