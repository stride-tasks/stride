use flutter_rust_bridge::frb;
use stride_plugin_manager::manifest::{
    ManifestEvent, ManifestPermission, PluginManifestInstance, PluginState,
};

#[frb(sync)]
#[must_use]
pub fn plugin_instance_manifest_name(manifest: &PluginManifestInstance) -> String {
    manifest.name().to_string()
}
#[frb(sync)]
#[must_use]
pub fn plugin_instance_manifest_enabled(manifest: &PluginManifestInstance) -> bool {
    manifest.state.is_enabled()
}
#[frb(sync)]
#[must_use]
pub fn plugin_instance_manifest_disabled_reason(
    manifest: &PluginManifestInstance,
) -> Option<String> {
    match &manifest.state {
        PluginState::Enable => None,
        PluginState::Disable { reason } => reason.as_deref().map(ToString::to_string),
    }
}
#[frb(sync)]
#[must_use]
pub fn plugin_instance_manifest_permission(
    manifest: &PluginManifestInstance,
) -> ManifestPermission {
    manifest.manifest.permission.clone()
}
#[frb(sync)]
#[must_use]
pub fn plugin_instance_manifest_event(manifest: &PluginManifestInstance) -> ManifestEvent {
    *manifest.events()
}
