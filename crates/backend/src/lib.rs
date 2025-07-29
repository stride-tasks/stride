//! Stride's backend implementations.

#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use stride_core::{
    backend::{Config, Schema},
    state::KnownPaths,
};
use stride_database::Database;
use uuid::Uuid;

pub mod error;
pub mod registry;

pub use error::{Error, Result};

pub trait BackendHandler: Debug + Sync + Send {
    // TODO: encapsulate name in a newtype, to restrict to ascii, no space, etc.
    fn name(&self) -> Box<str>;

    fn root_path(&self, repository_uuid: Uuid, known_paths: &KnownPaths) -> PathBuf {
        known_paths
            .backend_path(repository_uuid)
            .join(self.name().as_ref())
    }

    fn config_schema(&self) -> Schema;
    fn create(
        &self,
        config: &Config,
        path: &Path,
        known_paths: &KnownPaths,
    ) -> Result<Box<dyn Backend>>;
}

/// This is the main trait, defining a "Backend".
/// A backend governs how tasks are synced.
pub trait Backend {
    fn handler() -> Box<dyn BackendHandler>
    where
        Self: Sized;

    fn sync(&mut self, db: &mut Database) -> Result<()>;
}
