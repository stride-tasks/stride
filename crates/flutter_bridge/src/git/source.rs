use crate::git::known_hosts::{KnownHosts, KnownHostsError};
use git2::{CertificateCheckStatus, Cred, ErrorClass, ErrorCode, RemoteCallbacks};
use std::{io::ErrorKind, path::PathBuf};
use thiserror::Error;
use uuid::Uuid;
pub struct GitSource {
    /// Location of the repository.
    path: PathBuf,
}

impl GitSource {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn add_files(&self) {}

    pub fn commit(&self) {}

    pub fn push(&self) {}
}
