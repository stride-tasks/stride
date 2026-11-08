//! Stride's generic sqlite helpers: migrations and key-value storage.

#![allow(clippy::missing_errors_doc)]

mod error;
mod migration;
mod storage;

pub use error::{Error, Result};
pub use migration::{Migration, apply_migrations};
pub use storage::{StorageDatabase, StorageMigration, StorageTransaction, parse_key, validate_key};

/// Re-export so users of this crate use the same sqlite version.
pub use rusqlite;
