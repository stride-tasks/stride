//! Stride's crdt library.

pub use error::{Error, Result};

pub mod actor;
pub mod change;
pub mod difference;
pub mod hlc;
pub mod version_vector;

mod error;
