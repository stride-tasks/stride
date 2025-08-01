//! Core components used in `stride`.

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::doc_markdown)]

/// flutter_rust_bridge:ignore
#[cfg(feature = "backend")]
pub mod backend;

/// flutter_rust_bridge:ignore
pub mod state;

pub mod constant;
pub mod event;
pub mod task;
