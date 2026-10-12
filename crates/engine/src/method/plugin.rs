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
