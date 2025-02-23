use flutter_rust_bridge::frb;
use stride_core::event::HostEvent;
use stride_plugin_manager::{
    manifest::{ManifestEvents, ManifestPermissions, PluginInstanceManifest, PluginState},
    PluginManager,
};

use crate::{ErrorKind, RustError};

pub fn plugin_manifests(
    plugin_manager: &PluginManager,
) -> Result<Vec<PluginInstanceManifest>, RustError> {
    Ok(plugin_manager
        .list()?
        .iter()
        .map(|plugin| plugin.manifest.clone())
        .collect::<Vec<_>>())
}

#[frb(sync)]
#[must_use]
pub fn plugin_instance_manifest_name(manifest: &PluginInstanceManifest) -> String {
    manifest.name().to_string()
}
#[frb(sync)]
#[must_use]
pub fn plugin_instance_manifest_enabled(manifest: &PluginInstanceManifest) -> bool {
    manifest.state.is_enabled()
}
#[frb(sync)]
#[must_use]
pub fn plugin_instance_manifest_disabled_reason(
    manifest: &PluginInstanceManifest,
) -> Option<String> {
    match &manifest.state {
        PluginState::Enable => None,
        PluginState::Disable { reason } => reason.as_deref().map(ToString::to_string),
    }
}
#[frb(sync)]
#[must_use]
pub fn plugin_instance_manifest_permissions(
    manifest: &PluginInstanceManifest,
) -> ManifestPermissions {
    manifest.permissions
}
#[frb(sync)]
#[must_use]
pub fn plugin_instance_manifest_events(manifest: &PluginInstanceManifest) -> ManifestEvents {
    *manifest.events()
}

pub fn plugin_manager_emit(
    plugin_manager: &mut PluginManager,
    event: HostEvent,
) -> Result<(), RustError> {
    Ok(plugin_manager
        .emit_event(&event)
        .map_err(ErrorKind::Plugin)?)
}

pub fn plugin_manager_import(
    plugin_manager: &mut PluginManager,
    filepath: String,
) -> Result<(), RustError> {
    Ok(plugin_manager
        .import(&filepath)
        .map_err(ErrorKind::Plugin)?)
}

pub fn plugin_manager_remove(
    plugin_manager: &mut PluginManager,
    plugin_name: String,
) -> Result<bool, RustError> {
    Ok(plugin_manager
        .remove(&plugin_name)
        .map_err(ErrorKind::Plugin)?)
}

pub fn plugin_manager_disable(
    plugin_manager: &mut PluginManager,
    plugin_name: String,
    reason: Option<String>,
) -> Result<bool, RustError> {
    Ok(plugin_manager
        .disable(&plugin_name, reason)
        .map_err(ErrorKind::Plugin)?)
}

pub fn plugin_manager_toggle(
    plugin_manager: &mut PluginManager,
    plugin_name: String,
) -> Result<bool, RustError> {
    Ok(plugin_manager
        .toggle(&plugin_name)
        .map_err(ErrorKind::Plugin)?)
}
