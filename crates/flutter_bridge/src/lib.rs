//! Stride flutter bindings.

// TODO: Remove lint allows
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::similar_names)]
#![allow(clippy::module_name_repetitions)]

pub mod api;

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
#[allow(let_underscore_drop)]
#[allow(clippy::match_same_arms)]
#[rustfmt::skip]
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
