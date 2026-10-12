use std::{path::Path, sync::Arc};

use crate::{Engine, Result, TypedCommandHandler};

#[derive(Debug, Clone, Copy)]
pub struct PluginImportHandler;

impl TypedCommandHandler for PluginImportHandler {
    type Method = crate::api::PluginImportMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::PluginImportMethodResult> {
        engine
            .lock_plugin_manager()
            .import(Path::new(method.file_path.as_ref()))?;

        Ok(crate::api::PluginImportMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PluginRemoveHandler;

impl TypedCommandHandler for PluginRemoveHandler {
    type Method = crate::api::PluginRemoveMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::PluginRemoveMethodResult> {
        engine.lock_plugin_manager().remove(&method.plugin_name)?;

        Ok(crate::api::PluginRemoveMethodResult {})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PluginDisableHandler;

impl TypedCommandHandler for PluginDisableHandler {
    type Method = crate::api::PluginDisableMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::PluginDisableMethodResult> {
        engine
            .lock_plugin_manager()
            .disable(&method.plugin_name, method.reason.map(String::from))?;

        Ok(crate::api::PluginDisableMethodResult {})
    }
}
