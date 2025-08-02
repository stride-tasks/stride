use flutter_rust_bridge::frb;
use stride_backend_git::known_hosts::HostKeyType;

#[frb(sync)]
pub fn host_key_type_name(key_type: HostKeyType) -> String {
    key_type.name().to_string()
}
