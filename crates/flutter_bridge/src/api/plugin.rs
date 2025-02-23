use stride_plugin_manager::{
    manifest::{ManifestEvents, PluginInstanceManifest},
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

/// flutter_rust_bridge:sync
#[must_use]
pub fn plugin_instance_manifest_name(manifest: &PluginInstanceManifest) -> String {
    manifest.name().to_string()
}
/// flutter_rust_bridge:sync
#[must_use]
pub fn plugin_instance_manifest_events(manifest: &PluginInstanceManifest) -> ManifestEvents {
    *manifest.events()
}

pub fn plugin_manager_import(
    plugin_manager: &mut PluginManager,
    filepath: String,
) -> Result<(), RustError> {
    Ok(plugin_manager
        .import(&filepath)
        .map_err(ErrorKind::Plugin)?)
}
