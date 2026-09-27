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
    schema::{AdditionalProperties, Schema, SchemaType},
};

pub fn generate_rust(protocol_dir: &Path) -> Result<String> {
    let nodes = parse(protocol_dir)?;
    Ok(render_rust(&nodes))
}

pub fn generate_dart(protocol_dir: &Path) -> Result<String> {
    let nodes = parse(protocol_dir)?;
    Ok(render_dart(&nodes))
}
