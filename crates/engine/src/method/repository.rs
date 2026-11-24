use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use stride_api::Config;
use uuid::Uuid;

use crate::{Engine, Result, TypedCommandHandler};

#[derive(Debug, Clone, Copy)]
pub struct RepositoryAddHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryAddHandler {
    type Method = crate::api::RepositoryAddMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryAddMethodResult> {
        let repository_id = Uuid::now_v7();
        let descriptor = crate::api::RepositoryDescriptor {
            id: repository_id,
            name: method.name,
        };

        engine.open_repository(repository_id).await?;
        engine
            .known_paths()
            .write_repository_descriptor(&descriptor)?;

        Ok(crate::api::RepositoryAddMethodResult { repository_id })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryUpdateHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryUpdateHandler {
    type Method = crate::api::RepositoryUpdateMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryUpdateMethodResult> {
        engine
            .known_paths()
            .write_repository_descriptor(&method.descriptor)?;

        Ok(crate::api::RepositoryUpdateMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryListHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryListHandler {
    type Method = crate::api::RepositoryListMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        _: Self::Method,
    ) -> Result<crate::api::RepositoryListMethodResult> {
        let repositories_path = engine.known_paths().repositories_path();
        let entries = match std::fs::read_dir(repositories_path) {
            Ok(entries) => entries,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(crate::api::RepositoryListMethodResult {
                    repositories: Vec::new(),
                });
            }
            Err(error) => return Err(error.into()),
        };

        let mut repositories = Vec::new();
        for entry in entries {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }

            let file_name = entry.file_name();
            let Some(name) = file_name.to_str() else {
                continue;
            };

            let Some(uuid) = Uuid::parse_str(name).ok() else {
                continue;
            };

            let descriptor_path = entry.path().join("descriptor.toml");
            let contents = match std::fs::read_to_string(descriptor_path) {
                Ok(contents) => contents,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    let descriptor = crate::api::RepositoryDescriptor {
                        id: uuid,
                        name: None,
                    };
                    repositories.push(descriptor);
                    continue;
                }
                Err(error) => {
                    log::error!("Failed to read descriptor file: {error}");
                    continue;
                }
            };
            let descriptor =
                toml::from_str(&contents).map_err(|error| crate::Error::Other(Box::new(error)))?;
            repositories.push(descriptor);
        }

        repositories.sort_by_key(|repository: &crate::api::RepositoryDescriptor| repository.id);
        Ok(crate::api::RepositoryListMethodResult { repositories })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryBackendAddHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryBackendAddHandler {
    type Method = crate::api::RepositoryBackendAddMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryBackendAddMethodResult> {
        let handler = engine.backends().get_or_error(&method.backend)?;
        let name = handler.name();
        let backend_id = Uuid::now_v7();

        let repository = engine.open_repository(method.repository_id).await?;
        repository
            .lock_database()
            .await
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

#[async_trait]
impl TypedCommandHandler for RepositoryBackendRemoveHandler {
    type Method = crate::api::RepositoryBackendRemoveMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryBackendRemoveMethodResult> {
        let repository = engine.open_repository(method.repository_id).await?;
        repository
            .lock_database()
            .await
            .delete_backend(method.backend_id)?;

        Ok(crate::api::RepositoryBackendRemoveMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryBackendToggleHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryBackendToggleHandler {
    type Method = crate::api::RepositoryBackendToggleMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryBackendToggleMethodResult> {
        let repository = engine.open_repository(method.repository_id).await?;
        let new_state = repository
            .lock_database()
            .await
            .toggle_backend(method.backend_id)?;

        Ok(crate::api::RepositoryBackendToggleMethodResult { state: new_state })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryBackendGetHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryBackendGetHandler {
    type Method = crate::api::RepositoryBackendGetMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryBackendGetMethodResult> {
        let repository = engine.open_repository(method.repository_id).await?;
        let backend_instances = repository.lock_database().await.backends()?;

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

#[async_trait]
impl TypedCommandHandler for RepositoryBackendSetHandler {
    type Method = crate::api::RepositoryBackendSetMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryBackendSetMethodResult> {
        let repository = engine.open_repository(method.repository_id).await?;
        repository
            .lock_database()
            .await
            .update_backend(&method.backend_instance)?;

        Ok(crate::api::RepositoryBackendSetMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryBackendListHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryBackendListHandler {
    type Method = crate::api::RepositoryBackendListMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryBackendListMethodResult> {
        let repository = engine.open_repository(method.repository_id).await?;
        let backend_instances = repository.lock_database().await.backends()?;

        Ok(crate::api::RepositoryBackendListMethodResult {
            backends: backend_instances,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryRemoveHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryRemoveHandler {
    type Method = crate::api::RepositoryRemoveMethod;

    async fn handle(
        &self,
        context: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryRemoveMethodResult> {
        context.remove_repository(method.repository_id).await?;
        Ok(crate::api::RepositoryRemoveMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositorySyncHandler;

#[async_trait]
impl TypedCommandHandler for RepositorySyncHandler {
    type Method = crate::api::RepositorySyncMethod;

    async fn handle(
        &self,
        context: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositorySyncMethodResult> {
        let repository = context.open_repository(method.id).await?;

        let task = context.clone().runtime().spawn(async move {
            context.notify(Box::new(crate::api::RepositorySyncStartedNotification {
                repository_id: method.id,
            }));

            let changes_result = repository.sync(&context).await;

            context.notify(Box::new(crate::api::RepositorySyncStoppedNotification {
                repository_id: method.id,
                error: changes_result
                    .as_deref()
                    .err()
                    .map(|err| err.to_string().into_boxed_str()),
            }));

            if let Ok(changes) = changes_result {
                context.notify(Box::new(crate::api::RepositoryChangedNotification {
                    repository_id: method.id,
                    changes,
                }));
            }
        });

        // Detach the task so it runs independently and does not block the current thread.
        task.detach();

        Ok(crate::api::RepositorySyncMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryTagListHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryTagListHandler {
    type Method = crate::api::RepositoryTagListMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryTagListMethodResult> {
        let repository = engine.open_repository(method.id).await?;
        let tags = repository.lock_database().await.used_tags()?;
        Ok(crate::api::RepositoryTagListMethodResult { tags })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryProjectListHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryProjectListHandler {
    type Method = crate::api::RepositoryProjectListMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryProjectListMethodResult> {
        let repository = engine.open_repository(method.id).await?;
        let projects = repository.lock_database().await.used_projects()?;

        Ok(crate::api::RepositoryProjectListMethodResult { projects })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryExportHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryExportHandler {
    type Method = crate::api::RepositoryExportMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryExportMethodResult> {
        let repository = engine.open_repository(method.repository_id).await?;
        let mut tasks = repository.lock_database().await.all_tasks()?;
        tasks.sort_unstable_by_key(|task| task.id);

        Ok(crate::api::RepositoryExportMethodResult { tasks })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryImportHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryImportHandler {
    type Method = crate::api::RepositoryImportMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryImportMethodResult> {
        let repository = engine.open_repository(method.repository_id).await?;
        let imported = method.tasks.len();
        let mut database = repository.lock_database().await;
        let mut transaction = database.task_transaction()?;
        let mut dependencies = Vec::with_capacity(imported);

        for mut task in method.tasks {
            let id = task.id;
            dependencies.push((id, std::mem::take(&mut task.depends)));
            if transaction.task_by_id(id)?.is_some() {
                transaction.update_task_with(id, |_| Ok(task))?;
            } else {
                transaction.insert_task(&task)?;
            }
        }

        for (id, depends) in dependencies {
            transaction.update_task_with(id, |mut task| {
                task.depends = depends;
                Ok(task)
            })?;
        }

        transaction.commit()?;
        Ok(crate::api::RepositoryImportMethodResult { imported })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryTaskAddHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryTaskAddHandler {
    type Method = crate::api::RepositoryTaskAddMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryTaskAddMethodResult> {
        let repository = engine.open_repository(method.repository_id).await?;
        let mut database = repository.lock_database().await;
        let mut transaction = database.task_transaction()?;
        transaction.insert_task(&method.task)?;
        transaction.commit()?;
        drop(database);

        engine.notify(Box::new(crate::api::RepositoryTaskAddedNotification {
            repository_id: method.repository_id,
            task: method.task,
        }));

        Ok(crate::api::RepositoryTaskAddMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryTaskUpdateHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryTaskUpdateHandler {
    type Method = crate::api::RepositoryTaskUpdateMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryTaskUpdateMethodResult> {
        let repository = engine.open_repository(method.repository_id).await?;
        let mut task = method.task;
        task.modified = Some(Utc::now());

        let mut database = repository.lock_database().await;
        let mut transaction = database.task_transaction()?;
        let mut previous = None;
        let current = transaction.update_task_with(task.id, |existing| {
            previous = Some(existing);
            Ok(task)
        })?;
        transaction.commit()?;

        engine.notify(Box::new(crate::api::RepositoryTaskUpdatedNotification {
            repository_id: method.repository_id,
            previous: previous.unwrap(),
            current,
        }));

        Ok(crate::api::RepositoryTaskUpdateMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryTaskRemoveHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryTaskRemoveHandler {
    type Method = crate::api::RepositoryTaskRemoveMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryTaskRemoveMethodResult> {
        let repository = engine.open_repository(method.repository_id).await?;
        let mut database = repository.lock_database().await;
        let mut transaction = database.task_transaction()?;
        transaction.delete_task(method.task_id)?;
        transaction.commit()?;

        engine.notify(Box::new(crate::api::RepositoryTaskRemovedNotification {
            repository_id: method.repository_id,
            task_id: method.task_id,
        }));

        Ok(crate::api::RepositoryTaskRemoveMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryTaskQueryHandler;

#[async_trait]
impl TypedCommandHandler for RepositoryTaskQueryHandler {
    type Method = crate::api::RepositoryTaskQueryMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::RepositoryTaskQueryMethodResult> {
        let repository = engine.open_repository(method.repository_id).await?;
        let tasks = repository.lock_database().await.task_query(&method.query)?;

        Ok(crate::api::RepositoryTaskQueryMethodResult { tasks })
    }
}
