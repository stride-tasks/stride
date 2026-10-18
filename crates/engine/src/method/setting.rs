use std::sync::Arc;

use crate::{Engine, Error, Result, TypedCommandHandler};

#[derive(Debug, Clone, Copy)]
pub struct SettingGetHandler;

impl TypedCommandHandler for SettingGetHandler {
    type Method = crate::api::SettingGetMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        _method: Self::Method,
    ) -> Result<crate::api::SettingGetMethodResult> {
        let path = engine.known_paths().setting_filepath();
        if !path.exists() {
            return Ok(crate::api::SettingGetMethodResult {
                setting: crate::api::Setting::default(),
            });
        }

        let contents = std::fs::read_to_string(path)?;
        let setting = toml::from_str(&contents).map_err(|error| Error::Other(Box::new(error)))?;

        Ok(crate::api::SettingGetMethodResult { setting })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SettingSetHandler;

impl TypedCommandHandler for SettingSetHandler {
    type Method = crate::api::SettingSetMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        method: Self::Method,
    ) -> Result<crate::api::SettingSetMethodResult> {
        let support_path = &engine.known_paths().support;
        std::fs::create_dir_all(support_path)?;

        let contents = toml::to_string_pretty(&method.setting)
            .map_err(|error| Error::Other(Box::new(error)))?;
        std::fs::write(engine.known_paths().setting_filepath(), contents)?;

        Ok(crate::api::SettingSetMethodResult {})
    }
}
