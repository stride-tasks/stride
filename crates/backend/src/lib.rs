//! Stride's backend implementations.

#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use base64::{DecodeError, Engine};
use stride_database::Database;
use uuid::Uuid;

pub mod error;

pub mod config;
pub mod git;
pub mod taskchampion;

pub use error::{Error, Result};

/// This is the main trait, defining a "Backend".
/// A backend governs how tasks are synced.
pub trait Backend {
    fn sync(&mut self, db: &mut Database) -> Result<()>;
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
