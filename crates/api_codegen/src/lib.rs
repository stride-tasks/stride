//! This crate generates code from a JSON schema.

use crate::{
    parser::parse,
    render::{render_dart, render_rust},
};
use std::path::Path;

mod error;
pub mod node;
pub mod parser;
pub mod render;
mod schema;

#[cfg(test)]
mod tests;

pub use crate::{
    error::{Error, Result},
    schema::{AdditionalProperties, Schema, SchemaConcreteType},
};

/// Generate Rust code from the API schema located in `api_dir`.
///
/// # Errors
/// Returns an error if the API schema could not be parsed or
/// if the Rust code could not be generated.
pub fn generate_rust(api_dir: &Path) -> Result<String> {
    let nodes = parse(api_dir)?;
    Ok(render_rust(&nodes))
}

/// Generate Dart code from the API schema located in `api_dir`.
///
/// # Errors
///
/// Returns an error if the API schema could not be parsed or
/// if the Dart code could not be generated.
pub fn generate_dart(api_dir: &Path) -> Result<String> {
    let nodes = parse(api_dir)?;
    Ok(render_dart(&nodes))
}
