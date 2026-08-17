use std::sync::Arc;

use crate::known_hosts::{Host, KnownHosts};

use stride_api as api;

#[derive(Debug, Clone, Copy)]
pub struct SshHostAddHandler;

#[derive(Debug, serde::Deserialize)]
struct SshHostAddArgs {
    host: Host,
}

impl api::CommandHandler for SshHostAddHandler {
    fn handle(&self, _: Arc<dyn api::Context>, args: api::Value) -> api::Result<Box<str>> {
        let args = serde_json::to_string(&args).map_err(Box::new)?;
        let ssh_host_add_args: SshHostAddArgs = serde_json::from_str(&args).map_err(Box::new)?;
        let mut known_hosts = KnownHosts::read_standard_file().map_err(Box::new)?;
        known_hosts.add(ssh_host_add_args.host);
        known_hosts.write_standard_file().map_err(Box::new)?;
        Ok("".into())
    }
}
