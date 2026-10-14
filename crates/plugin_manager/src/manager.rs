use std::{
    collections::{HashMap, VecDeque},
    path::{Path, PathBuf},
};

use stride_api::{self as api, Method, PluginManifestInstance, PluginState};
use wasmi::{Config, Engine};

use crate::{Error, Plugin, PluginCallQueue, Result};

#[derive(Debug, Default)]
pub struct PluginManager {
    pub(crate) plugins_path: PathBuf,
    pub(crate) plugins: HashMap<Box<str>, Plugin>,

    pub(crate) engine: Engine,

    pub(crate) host_messages: VecDeque<(Box<str>, api::Value)>,
    pub(crate) plugin_calls: VecDeque<PluginCallQueue>,
}

impl PluginManager {
    pub fn new(plugins_path: &Path) -> Result<Self> {
        if !plugins_path.exists() {
            std::fs::create_dir_all(plugins_path)?;
        }

        let mut config = Config::default();
        config.consume_fuel(true);
        let engine = Engine::new(&config);

        Ok(Self {
            plugins_path: plugins_path.to_path_buf(),
            plugins: HashMap::new(),

            engine,
            host_messages: VecDeque::new(),
            plugin_calls: VecDeque::new(),
        })
    }

    pub fn load(&mut self) -> Result<()> {
        let entries = self.plugins_path.read_dir()?;

        let mut plugins = HashMap::new();
        for entry in entries {
            let Ok(entry) = entry else {
                continue;
            };

            let Ok(name) = entry.file_name().into_string() else {
                continue;
            };

            let plugin_path = self.plugins_path.join(name);
            let source_path = plugin_path.join("source");
            let manifest_path = source_path.join("manifest.toml");

            let manifest_content = std::fs::read_to_string(&manifest_path)?;

            let instance: PluginManifestInstance =
                toml::from_str(&manifest_content).map_err(Error::Deserialize)?;

            plugins.insert(instance.manifest.name.clone(), Plugin { instance });
        }

        self.plugins = plugins;
        Ok(())
    }

    /// flutter_rust_bridge:ignore
    pub fn list(&self) -> impl Iterator<Item = &Plugin> {
        self.plugins.values()
    }

    pub fn disable(&mut self, plugin_name: &str, reason: Option<Box<str>>) -> Result<bool> {
        let Some(plugin) = self.plugins.get_mut(plugin_name) else {
            return Ok(false);
        };

        match plugin.instance.state {
            PluginState::Disable { .. } => return Ok(false),
            PluginState::Enable {} => {}
        }

        plugin.instance.state = PluginState::Disable { reason };

        let plugin_path = self
            .plugins_path
            .join(plugin.instance.manifest.name.as_ref());
        let source_path = plugin_path.join("source");
        std::fs::create_dir_all(&source_path)?;

        let manifest_path = source_path.join("manifest.toml");
        let manifest_content =
            toml::to_string_pretty(&plugin.instance).map_err(Error::Serialize)?;
        std::fs::write(&manifest_path, manifest_content)?;

        Ok(true)
    }

    pub fn remove(&mut self, plugin_name: &str) -> Result<bool> {
        let Some(plugin) = self.plugins.remove(plugin_name) else {
            return Ok(false);
        };

        let plugin_path = self
            .plugins_path
            .join(plugin.instance.manifest.name.as_ref());
        std::fs::remove_dir_all(plugin_path)?;

        Ok(true)
    }

    pub fn process_plugin_call(&mut self) -> Option<(Box<str>, api::Value)> {
        loop {
            let mut calls = self.plugin_calls.pop_front()?;
            let Some(call) = calls.calls.pop_front() else {
                continue;
            };

            if !calls.calls.is_empty() {
                self.plugin_calls.push_back(PluginCallQueue {
                    plugin_name: calls.plugin_name.clone(),
                    calls: calls.calls,
                });
            }

            return Some((calls.plugin_name, call));
        }
    }

    pub fn method_permission_error(&self, plugin_name: &str, method: &str) -> Option<Box<str>> {
        let plugin = self.plugin(plugin_name)?;
        if !matches!(plugin.instance.state, PluginState::Enable {}) {
            return Some("plugin is disabled".into());
        }

        let task_permission = plugin.instance.manifest.permission.task.as_ref();
        let (allowed, permission) = match method {
            api::RepositoryTaskAddMethod::NAME => (
                task_permission.is_some_and(|task| task.create),
                "task.create",
            ),
            api::RepositoryTaskUpdateMethod::NAME => (
                task_permission.is_some_and(|task| task.modify),
                "task.modify",
            ),
            api::RepositoryTaskRemoveMethod::NAME => (
                task_permission.is_some_and(|task| task.remove),
                "task.remove",
            ),
            api::RepositoryTaskQueryMethod::NAME => {
                (task_permission.is_some_and(|task| task.query), "task.query")
            }
            api::RepositorySyncMethod::NAME => {
                (task_permission.is_some_and(|task| task.sync), "task.sync")
            }
            _ => return Some(format!("unsupported API method '{method}'").into()),
        };

        if allowed {
            None
        } else {
            Some(format!("missing '{permission}' permission").into())
        }
    }
}
