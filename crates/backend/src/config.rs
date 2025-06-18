use std::path::Path;

use stride_core::config::{Schema, SchemaError};

pub trait FromSchema: Sized {
    fn from_schema(schema: &Schema, root_path: &Path) -> Result<Self, SchemaError>;
    fn default_schema() -> Schema;
}
