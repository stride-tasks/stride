use std::{path::Path, process::ExitCode};

use anyhow::{Context, bail};
use stride_backend::registry::Registry;
use stride_core::{
    backend::{BackendRecord, Config, Value},
    state::KnownPaths,
};
use stride_database::Database;
use uuid::Uuid;

use crate::cli::BackendCommand;

// TODO: Remove lints.
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::too_many_lines)]
pub(crate) fn handle_command(
    command: Option<&BackendCommand>,
    backend_registry: &Registry,
    database: &mut Database,
    repository_path: &Path,
    known_paths: &KnownPaths,
) -> anyhow::Result<ExitCode> {
    match command {
        None | Some(BackendCommand::List) => {
            let backends = database.backends()?;
            for (i, backend) in backends.iter().enumerate() {
                println!("{i:2}. {}", backend.name);
            }
        }
        Some(BackendCommand::New { backend_name: name }) => {
            let handler = backend_registry.get_or_error(name.as_str())?;
            let record = BackendRecord {
                id: Uuid::now_v7(),
                enabled: true,
                name: handler.name(),
                config: Config::default(),
            };
            database.add_backend(&record)?;
        }
        Some(BackendCommand::Config {
            backend_name: name,
            property_name,
            unset,
            property_value,
        }) => {
            let mut backends = database.backends()?;

            let backend = backends
                .iter_mut()
                .find(|backend_record| backend_record.name.contains(name))
                .with_context(|| format!("Could not find field with name {name}"))?;

            let handler = backend_registry.get_or_error(name.as_str())?;

            let schema = handler.config_schema();
            let config = backend.config.align(&schema)?;
            if config != backend.config {
                backend.config = config;
                database.update_backend(backend)?;
            }

            let Some(property_name) = property_name else {
                for (id, schema_field) in &schema.fields {
                    let required = if schema_field.value.as_value().is_some() {
                        "*"
                    } else {
                        ""
                    };
                    let value = backend
                        .config
                        .get(id)
                        .cloned()
                        .or_else(|| schema_field.value.as_value());

                    println!(
                        "{id}: {}{required} = {}",
                        schema_field.value.as_type_name(),
                        value.map_or_else(
                            || String::from("none").into(),
                            |value: Value| value.as_value_string()
                        )
                    );
                }
                return Ok(ExitCode::SUCCESS);
            };

            let Some(schema_field) = schema.field(property_name) else {
                bail!("property is not in the schema: {property_name}");
            };

            let Some(property_value) = property_value else {
                if *unset {
                    backend.config.unset(property_name);
                    return Ok(ExitCode::SUCCESS);
                }

                let value = backend.config.get(property_name);
                if let Some(value) = value {
                    println!("{value}");
                }

                let has_value = value.is_some();
                database.update_backend(backend)?;

                return Ok(ExitCode::from(u8::from(!has_value)));
            };

            backend
                .config
                .set(schema_field, property_name, property_value)?;

            database.update_backend(backend)?;
        }
        Some(BackendCommand::Remove { backend_name }) => {
            let backends = database.backends()?;

            let backend = backends
                .iter()
                .find(|backend_record| backend_record.name.contains(backend_name))
                .with_context(|| format!("Could not find field with name {backend_name}"))?;

            database.delete_backend(backend.id)?;
        }
        Some(BackendCommand::Invoke {
            backend_name,
            method_id,
        }) => {
            let mut backends = database.backends()?;
            let backend = backends
                .iter_mut()
                .find(|backend_record| backend_record.name.contains(backend_name))
                .with_context(|| format!("Could not find field with name {backend_name}"))?;

            let handler = backend_registry.get_or_error(backend_name.as_str())?;

            let schema = handler.config_schema();
            let config = backend.config.align(&schema)?;
            let Some(method_id) = method_id else {
                for (id, method) in schema.methods {
                    println!("{id}: {}", method.name);
                }
                return Ok(ExitCode::SUCCESS);
            };

            let Some((_, _method)) = schema
                .methods
                .iter()
                .find(|(id, _)| id.as_ref() == method_id.as_str())
            else {
                bail!("Unknown method: {method_id}");
            };

            let path = repository_path
                .join("backend")
                .join(handler.name().as_ref())
                .join(backend.id.to_string());

            let config = config.fill(&schema)?;
            let mut backend = handler.create(&config, &path, known_paths)?;

            backend.invoke(method_id, database)?;
        }
    }
    Ok(ExitCode::SUCCESS)
}
