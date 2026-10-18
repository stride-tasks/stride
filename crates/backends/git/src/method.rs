use std::sync::Arc;

use crate::known_hosts::KnownHosts;

use stride_engine::{Engine, TypedCommandHandler, api};

#[derive(Debug, Clone, Copy)]
pub struct SshHostAddHandler;

impl TypedCommandHandler for SshHostAddHandler {
    type Method = api::SshHostAddMethod;

    fn handle(
        &self,
        engine: Arc<Engine>,
        args: Self::Method,
    ) -> stride_engine::Result<api::SshHostAddMethodResult> {
        let known_hosts_path = engine
            .known_paths()
            .support
            .join(".ssh")
            .join("known_hosts");
        let mut known_hosts = KnownHosts::read_file(&known_hosts_path).map_err(Box::new)?;
        known_hosts.add_host(args.host);
        known_hosts
            .write_file(&known_hosts_path)
            .map_err(Box::new)?;
        Ok(api::SshHostAddMethodResult {})
    }
}
