//! Stride flutter bindings.

// TODO: Remove lint allows
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::similar_names)]
#![allow(clippy::module_name_repetitions)]

use base64::{DecodeError, Engine};
use uuid::Uuid;

pub mod api;
pub mod git;
pub mod key_store;

pub use api::error::{ErrorKind, RustError};

pub use stride_core::task;
pub use stride_plugin_manager as plugin;

#[allow(clippy::unreadable_literal)]
#[allow(unused_qualifications)]
#[allow(clippy::redundant_else)]
#[allow(unreachable_pub)]
#[allow(clippy::wildcard_imports)]
#[allow(clippy::semicolon_if_nothing_returned)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::uninlined_format_args)]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_lossless)]
#[allow(clippy::too_many_lines)]
#[allow(let_underscore_drop)]
#[allow(clippy::match_same_arms)]
#[rustfmt::skip]
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */

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
