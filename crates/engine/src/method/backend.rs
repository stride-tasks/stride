use std::sync::Arc;

use async_trait::async_trait;

use crate::{Engine, Result, TypedCommandHandler};

#[derive(Debug, Clone, Copy)]
pub struct BackendListHandler;

#[async_trait]
impl TypedCommandHandler for BackendListHandler {
    type Method = crate::api::BackendListMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        _: Self::Method,
    ) -> Result<crate::api::BackendListMethodResult> {
        let backends = engine
            .backends()
            .values()
            .map(|handler| crate::api::BackendDescriptor {
                name: handler.name(),
                schema: handler.config_schema(),
            })
            .collect::<Vec<_>>();

        Ok(crate::api::BackendListMethodResult { backends })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BackendGetHandler;

#[async_trait]
impl TypedCommandHandler for BackendGetHandler {
    type Method = crate::api::BackendGetMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::BackendGetMethodResult> {
        let backend_descriptor = engine.backends().get(&method.backend_name).map(|handler| {
            crate::api::BackendDescriptor {
                name: handler.name(),
                schema: handler.config_schema(),
            }
        });

        Ok(crate::api::BackendGetMethodResult { backend_descriptor })
    }
}
