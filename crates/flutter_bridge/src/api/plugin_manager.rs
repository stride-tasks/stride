#![allow(clippy::needless_pass_by_value)]

use std::{
    path::Path,
    sync::{LazyLock, Mutex},
};

use stride_api::PluginEvent as HostEvent;
use stride_plugin_manager::manifest::{PluginAction, PluginManifestInstance};

use crate::{ErrorKind, RustError, api::context::ENGINE, frb_generated::StreamSink};

pub(crate) static STREAM: LazyLock<Mutex<Option<StreamSink<()>>>> = LazyLock::new(Mutex::default);

pub fn create_stream(stream: StreamSink<()>) {
    let mut event_stream = STREAM.lock().unwrap();
    *event_stream = Some(stream);
}

pub fn emit(event: HostEvent, plugin_name: String) -> Result<(), RustError> {
    let mut event_stream = STREAM.lock().unwrap();
    if let Some(event_stream) = event_stream.as_mut() {
        event_stream.add(()).unwrap();
    }
    drop(event_stream);

    let mut plugin_manager = ENGINE.wait().lock_plugin_manager();
    Ok(plugin_manager
        .emit_event(Some(plugin_name.as_str()), &event)
        .map_err(ErrorKind::Plugin)?)
}
pub fn emit_broadcast(event: HostEvent) -> Result<(), RustError> {
    let mut event_stream = STREAM.lock().unwrap();
    if let Some(event_stream) = event_stream.as_mut() {
        event_stream.add(()).unwrap();
    }
    drop(event_stream);

    let mut plugin_manager = ENGINE.wait().lock_plugin_manager();
    Ok(plugin_manager
        .emit_event(None, &event)
        .map_err(ErrorKind::Plugin)?)
}
pub fn process_host_event() -> Result<bool, RustError> {
    let mut plugin_manager = ENGINE.wait().lock_plugin_manager();
    Ok(plugin_manager
        .process_host_event()
        .map_err(ErrorKind::Plugin)?)
}
pub fn process_plugin_event() -> Result<Option<PluginAction>, RustError> {
    let mut plugin_manager = ENGINE.wait().lock_plugin_manager();
    Ok(plugin_manager.process_plugin_event())
}

pub fn plugin_manifests() -> Result<Vec<PluginManifestInstance>, RustError> {
    let plugin_manager = ENGINE.wait().lock_plugin_manager();
    Ok(plugin_manager
        .list()
        .map(|plugin| plugin.instance.clone())
        .collect::<Vec<_>>())
}
pub fn parse_plugin(filepath: String) -> Result<PluginManifestInstance, RustError> {
    let plugin_manager = ENGINE.wait().lock_plugin_manager();
    plugin_manager
        .parse_plugin(Path::new(&filepath))
        .map(|plugin| plugin.instance)
        .map_err(ErrorKind::Plugin)
        .map_err(Into::into)
}
