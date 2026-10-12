use std::sync::Arc;

use stride_api::Config;
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
        repository
            .lock_database()
            .add_backend(&crate::api::BackendInstance {
                id: backend_id,
                name,
                state: crate::api::BackendInstanceState::Disabled,
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
pub struct RepositoryBackendGetHandler;

impl TypedCommandHandler for RepositoryBackendGetHandler {
    type Method = crate::api::RepositoryBackendGetMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryBackendGetMethodResult> {
        let repository = engine.open_repository(method.repository_id)?;
        let backend_instances = repository.lock_database().backends()?;

        let backend_instance = backend_instances
            .into_iter()
            .find(|backend| backend.id == method.backend_id);

        let backend_record = backend_instance.map(|instance| {
            let descriptor = engine.backends().get(&instance.name);
            let descriptor = descriptor.map(|handler| crate::api::BackendDescriptor {
                name: handler.name(),
                schema: handler.config_schema(),
            });

            crate::api::BackendRecord {
                instance,
                descriptor,
            }
        });

        Ok(crate::api::RepositoryBackendGetMethodResult { backend_record })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryBackendSetHandler;

impl TypedCommandHandler for RepositoryBackendSetHandler {
    type Method = crate::api::RepositoryBackendSetMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryBackendSetMethodResult> {
        let repository = engine.open_repository(method.repository_id)?;
        repository
            .lock_database()
            .update_backend(&method.backend_instance)?;

        Ok(crate::api::RepositoryBackendSetMethodResult {})
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
            backends: backend_instances,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryRemoveHandler;

impl TypedCommandHandler for RepositoryRemoveHandler {
    type Method = crate::api::RepositoryRemoveMethod;

    fn handle(
        &self,
        context: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryRemoveMethodResult> {
        context.remove_repository(method.repository_id)?;
        Ok(crate::api::RepositoryRemoveMethodResult {})
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

#[derive(Debug, Clone, Copy)]
pub struct RepositoryTaskAddHandler;

impl TypedCommandHandler for RepositoryTaskAddHandler {
    type Method = crate::api::RepositoryTaskAddMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryTaskAddMethodResult> {
        let repository = engine.open_repository(method.repository_id)?;
        let mut database = repository.lock_database();
        let mut transaction = database.transaction()?;
        transaction.insert_task(&method.task)?;
        transaction.commit()?;

        Ok(crate::api::RepositoryTaskAddMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryTaskRemoveHandler;

impl TypedCommandHandler for RepositoryTaskRemoveHandler {
    type Method = crate::api::RepositoryTaskRemoveMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryTaskRemoveMethodResult> {
        let repository = engine.open_repository(method.repository_id)?;
        let mut database = repository.lock_database();
        let mut transaction = database.transaction()?;
        transaction.delete_task(method.task_id)?;
        transaction.commit()?;

        Ok(crate::api::RepositoryTaskRemoveMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryTaskQueryHandler;

impl TypedCommandHandler for RepositoryTaskQueryHandler {
    type Method = crate::api::RepositoryTaskQueryMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryTaskQueryMethodResult> {
        let repository = engine.open_repository(method.repository_id)?;
        let tasks = repository.lock_database().task_query(&method.query)?;

        Ok(crate::api::RepositoryTaskQueryMethodResult { tasks })
    }
}

