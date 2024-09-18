// TODO: Remove lint allows
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::similar_names)]
#![allow(clippy::module_name_repetitions)]

use base64::Engine;
use uuid::Uuid;

pub mod api;
pub mod git;
pub mod key_store;
pub mod task;

pub use api::error::{ErrorKind, RustError};

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
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */

pub(crate) trait ToBase64 {
    fn to_base64(&self) -> String;
}

impl ToBase64 for Uuid {
    fn to_base64(&self) -> String {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(self.as_bytes())
    }
}
