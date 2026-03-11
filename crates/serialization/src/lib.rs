//! Stride's serialization crate.

mod blob;
mod error;

pub use crate::blob::{FromBlob, ToBlob};
pub use crate::error::{Error, Result};
