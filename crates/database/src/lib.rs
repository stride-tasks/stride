//! Stride's sqlite database wrapper library.

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

pub mod conversion;
mod error;
mod migrations;

mod database;

use conversion::{Sql, task_status_to_sql};
pub use error::{Error, Result};
pub mod operation;

use migrations::apply_migrations;

pub use database::Database;
