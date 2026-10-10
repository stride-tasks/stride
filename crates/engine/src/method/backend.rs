use std::sync::Arc;

use crate::{Engine, Result, TypedCommandHandler};

#[derive(Debug, Clone, Copy)]
pub struct BackendListHandler;

impl TypedCommandHandler for BackendListHandler {
    type Method = crate::api::BackendListMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        _: Self::Method,
    ) -> Result<crate::api::BackendListMethodResult> {
        let backends = engine
            .backends()
            .values()
            .map(|handler| crate::api::BackendDescriptor {
                name: handler.name().into(),
                schema: crate::api::Value::from_type(&handler.config_schema())
                    .to_string()
                    .into_boxed_str(),
            })
            .collect::<Vec<_>>();

        Ok(crate::api::BackendListMethodResult { backends })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BackendGetHandler;

impl TypedCommandHandler for BackendGetHandler {
    type Method = crate::api::BackendGetMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::BackendGetMethodResult> {
        let backend_descriptor = engine.backends().get(&method.backend_name).map(|handler| {
            crate::api::BackendDescriptor {
                name: handler.name().into(),
                schema: crate::api::Value::from_type(&handler.config_schema())
                    .to_string()
                    .into_boxed_str(),
            }
        });

        Ok(crate::api::BackendGetMethodResult { backend_descriptor })
    }
}
