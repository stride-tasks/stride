use crate::api::repository::Repository;
use std::sync::Arc;
use stride_api as api;

#[derive(Debug, Clone, Copy)]
pub struct RepositorySyncHandler;

impl api::TypedCommandHandler for RepositorySyncHandler {
    type Method = api::RepositorySyncMethod;

    fn handle(&self, context: Arc<dyn api::Context>, method: Self::Method) -> api::Result<()> {
        let mut repository = Repository::open(method.id).map_err(Box::new)?;
        let changes = repository.sync(&context).map_err(Box::new)?;

        context.notify(Box::new(api::RepositoryChangedNotification {
            repository_id: method.id,
            changes,
        }))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryTagListHandler;

impl api::TypedCommandHandler for RepositoryTagListHandler {
    type Method = api::RepositoryTagListMethod;

    fn handle(
        &self,
        _: Arc<dyn api::Context>,
        method: Self::Method,
    ) -> api::Result<api::RepositoryTagListMethodResult> {
        let repository = Repository::open(method.id).map_err(Box::new)?;
        let tags = repository
            .database()
            .lock()
            .unwrap()
            .used_tags()
            .map_err(Box::new)?;

        Ok(api::RepositoryTagListMethodResult { tags })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepositoryProjectListHandler;

impl api::TypedCommandHandler for RepositoryProjectListHandler {
    type Method = api::RepositoryProjectListMethod;

    fn handle(
        &self,
        _: Arc<dyn api::Context>,
        method: Self::Method,
    ) -> api::Result<api::RepositoryProjectListMethodResult> {
        let repository = Repository::open(method.id).map_err(Box::new)?;
        let projects = repository
            .database()
            .lock()
            .unwrap()
            .used_projects()
            .map_err(Box::new)?;

        Ok(api::RepositoryProjectListMethodResult { projects })
    }
}
