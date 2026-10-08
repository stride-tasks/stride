use std::sync::Arc;

use crate::known_hosts::KnownHosts;

use stride_engine::{Engine, TypedCommandHandler, api};

#[derive(Debug, Clone, Copy)]
pub struct SshHostAddHandler;

impl TypedCommandHandler for SshHostAddHandler {
    type Method = api::SshHostAddMethod;

    fn handle(
        &self,
        _: Arc<Engine>,
        args: Self::Method,
    ) -> stride_engine::Result<api::SshHostAddMethodResult> {
        let mut known_hosts = KnownHosts::read_standard_file().map_err(Box::new)?;
        known_hosts.add_host(args.host);
        known_hosts.write_standard_file().map_err(Box::new)?;
        Ok(api::SshHostAddMethodResult {})
    }
}
