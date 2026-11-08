use std::sync::Arc;

use async_trait::async_trait;
use stride_sqlite::StorageTransaction;
use uuid::Uuid;

use crate::{Engine, Error, Result, TypedCommandHandler};

/// Runs `f` on the storage of the given repository, or the global storage if there is none.
async fn with_storage<T>(
    engine: &Arc<Engine>,
    repository_id: Option<Uuid>,
    f: impl FnOnce(StorageTransaction<'_>) -> stride_sqlite::Result<T>,
) -> Result<T> {
    if let Some(repository_id) = repository_id {
        if !engine
            .known_paths()
            .database_filepath(repository_id)
            .exists()
        {
            return Err(Error::RepositoryNotFound { id: repository_id });
        }
        let repository = engine.open_repository(repository_id).await?;
        let mut database = repository.lock_database().await;
        let transaction = database.transaction()?;
        let result = f(StorageTransaction::new(transaction))?;
        Ok(result)
    } else {
        let mut database = engine.lock_global_database().await?;
        let transaction = database.transaction()?;
        let result = f(StorageTransaction::new(transaction))?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StorageListHandler;

#[async_trait]
impl TypedCommandHandler for StorageListHandler {
    type Method = crate::api::StorageListMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::StorageListMethodResult> {
        let items = with_storage(&engine, method.repository_id, |storage| storage.list()).await?;
        Ok(crate::api::StorageListMethodResult { items })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StorageGetHandler;

#[async_trait]
impl TypedCommandHandler for StorageGetHandler {
    type Method = crate::api::StorageGetMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::StorageGetMethodResult> {
        let items = with_storage(&engine, method.repository_id, |storage| {
            storage.get(&method.items)
        })
        .await?;
        Ok(crate::api::StorageGetMethodResult { items })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StorageSetHandler;

#[async_trait]
impl TypedCommandHandler for StorageSetHandler {
    type Method = crate::api::StorageSetMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::StorageSetMethodResult> {
        with_storage(&engine, method.repository_id, |storage| {
            storage.set(&method.key, &method.value)?;
            storage.commit()
        })
        .await?;

        engine.notify(Box::new(crate::api::StorageChangedNotification {
            repository_id: method.repository_id,
            key: method.key,
            value: method.value,
        }));
        Ok(crate::api::StorageSetMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StorageRemoveHandler;

#[async_trait]
impl TypedCommandHandler for StorageRemoveHandler {
    type Method = crate::api::StorageRemoveMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::StorageRemoveMethodResult> {
        let removed = with_storage(&engine, method.repository_id, |storage| {
            storage.remove(&method.items)
        })
        .await?;

        for key in removed {
            engine.notify(Box::new(crate::api::StorageRemovedNotification {
                repository_id: method.repository_id,
                key,
            }));
        }
        Ok(crate::api::StorageRemoveMethodResult {})
    }
}
