use flutter_rust_bridge::frb;
use stride_plugin_manager::manifest::{
    ManifestEvent, ManifestPermission, PluginInstanceManifest, PluginState,
};

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
pub fn plugin_instance_manifest_permission(
    manifest: &PluginInstanceManifest,
) -> ManifestPermission {
    manifest.permission.clone()
}
#[frb(sync)]
#[must_use]
pub fn plugin_instance_manifest_event(manifest: &PluginInstanceManifest) -> ManifestEvent {
    *manifest.events()
}
