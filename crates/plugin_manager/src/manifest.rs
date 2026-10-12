use serde::{Deserialize, Serialize};
use stride_core::event::PluginEvent;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PluginApi {
    V1,
}

pub type PluginName = String;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct ManifestEventTask {
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub create: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub modify: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub sync: bool,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct ManifestEventTimer {
    pub interval: u32,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
#[allow(clippy::struct_excessive_bools)]
pub struct ManifestPermissionTask {
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub create: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub modify: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub query: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub sync: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ManifestPermissionNetwork {
    pub urls: Vec<String>,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct ManifestPermissionStorage {
    pub max_size: u32,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct ManifestEvent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<ManifestEventTask>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer: Option<ManifestEventTimer>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ManifestPermission {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<ManifestPermissionTask>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<ManifestPermissionNetwork>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<ManifestPermissionStorage>,
}

/// flutter_rust_bridge:ignore
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PluginState {
    #[default]
    Enable,
    Disable {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<Box<str>>,
    },
}

impl PluginState {
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enable)
    }
}

/// flutter_rust_bridge:opaque
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PluginManifest {
    pub api: PluginApi,
    pub name: PluginName,

    #[serde(default)]
    pub event: ManifestEvent,

    #[serde(default)]
    pub permission: ManifestPermission,
}

/// flutter_rust_bridge:opaque
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PluginManifestInstance {
    #[serde(flatten)]
    pub manifest: PluginManifest,
    pub state: PluginState,
}

impl PluginManifestInstance {
    /// flutter_rust_bridge:sync
    #[must_use]
    pub fn name(&self) -> &str {
        &self.manifest.name
    }
    /// flutter_rust_bridge:sync
    #[must_use]
    pub fn events(&self) -> &ManifestEvent {
        &self.manifest.event
    }
}

/// flutter_rust_bridge:non_opaque
#[derive(Debug, Clone)]
pub enum PluginAction {
    Event {
        plugin_name: String,
        event: PluginEvent,
    },
    Disable {
        plugin_name: String,
        reason: String,
    },
}
