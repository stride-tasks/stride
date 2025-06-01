//! Stride's backend implementations.

#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use base64::{DecodeError, Engine};
use stride_database::Database;
use uuid::Uuid;

pub mod error;

pub mod git;
pub mod taskchampion;

pub use error::{Error, Result};

/// This is the main trait, defining a "Backend".
/// A backend governs how tasks are synced.
///
// TODO: Move this to separate crate (stride_backend)
// TODO: Ideally we shouldn't need to expose this to flutter.
pub trait Backend {
    // /* TODO(@bpeetz): I have no idea, what this function does <2024-10-25> */
    // fn unload(&mut self);

    // /// Add a [`Task`] to the Repository
    // fn add(&mut self, task: Task) -> Result<()>;
    // /// Remove a [`Task`], will return [`None`] if it did not exists
    // fn remove_by_uuid(&mut self, uuid: &Uuid) -> Result<Option<Task>>;
    // /// Remove an existing [`Task`], returning [`true`] if it was previously added
    // fn remove_by_task(&mut self, task: &Task) -> Result<bool>;

    // /// Try to get a [`Task`] by [`Uuid`]
    // fn task_by_uuid(&mut self, uuid: &Uuid) -> Result<Option<Task>>;

    // /// Ensure that all previous operations are written to disk.
    // fn commit(&mut self) -> Result<()>;

    // fn update(&mut self, task: &Task) -> Result<bool>;

    fn sync(&mut self, db: &mut Database) -> Result<()>;
    // fn clear(&mut self) -> Result<()>;

    // fn export(&mut self) -> Result<String>;
    // fn import(&mut self, content: &str) -> Result<()>;

    // fn query(&mut self, query: &TaskQuery) -> Result<Vec<Task>>;
}

pub(crate) fn base64_encode<T: AsRef<[u8]>>(input: T) -> String {
    fn inner(input: &[u8]) -> String {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input)
    }
    inner(input.as_ref())
}

pub(crate) fn base64_decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
    fn inner(input: &[u8]) -> Result<Vec<u8>, DecodeError> {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(input)
    }
    inner(input.as_ref())
}

pub(crate) trait ToBase64 {
    fn to_base64(&self) -> String;
}

impl ToBase64 for Uuid {
    fn to_base64(&self) -> String {
        base64_encode(self.as_bytes())
    }
}
