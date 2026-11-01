use std::sync::Arc;

use async_trait::async_trait;

use crate::{Engine, Result, TypedCommandHandler};

#[derive(Debug, Clone, Copy)]
pub struct LogEventHandler;

#[async_trait]
impl TypedCommandHandler for LogEventHandler {
    type Method = crate::api::LogEventMethod;

    async fn handle(
        &self,
        _: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::LogEventMethodResult> {
        match method.level {
            crate::api::LogLevel::Debug => log::debug!("{}", method.message),
            crate::api::LogLevel::Trace => log::trace!("{}", method.message),
            crate::api::LogLevel::Info => log::info!("{}", method.message),
            crate::api::LogLevel::Warn => log::warn!("{}", method.message),
            crate::api::LogLevel::Error => log::error!("{}", method.message),
        }
        Ok(crate::api::LogEventMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LogGetHandler;

#[async_trait]
impl TypedCommandHandler for LogGetHandler {
    type Method = crate::api::LogGetMethod;

    async fn handle(
        &self,
        engine: Arc<Engine>,
        _method: Self::Method,
    ) -> Result<crate::api::LogGetMethodResult> {
        let path = &engine.known_paths().logs;
        if !path.exists() {
            return Ok(crate::api::LogGetMethodResult {
                content: Box::default(),
            });
        }
        let content = std::fs::read_to_string(path).expect("could not read logs");
        Ok(crate::api::LogGetMethodResult {
            content: content.into(),
        })
    }
}
