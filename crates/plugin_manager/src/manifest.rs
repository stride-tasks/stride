use serde::{Deserialize, Serialize};

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
pub struct ManifestPermissionTask {
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
pub struct ManifestEvents {
    pub task: ManifestEventTask,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct ManifestPermissions {
    pub task: ManifestPermissionTask,
}

pub trait ManifestState: Sized {
    fn skip_serializing(&self) -> bool {
        true
    }
}

impl ManifestState for () {}

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

impl ManifestState for PluginState {
    fn skip_serializing(&self) -> bool {
        false
    }
}

impl PluginState {
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enable)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PluginManifest<T: ManifestState = ()> {
    pub api: PluginApi,
    pub name: PluginName,

    #[serde(default)]
    pub events: ManifestEvents,

    #[serde(default)]
    pub permissions: ManifestPermissions,

    #[serde(default)]
    #[serde(skip_serializing_if = "ManifestState::skip_serializing")]
    pub state: T,
}
