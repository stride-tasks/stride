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
