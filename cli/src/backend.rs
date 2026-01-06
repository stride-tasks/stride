use std::process::ExitCode;

use anyhow::{Context, bail};
use stride_core::{
    backend::{BackendRecord, Config},
    state::KnownPaths,
};
use stride_flutter_bridge::api::repository::Repository;
use uuid::Uuid;

use crate::{cli::BackendCommand, get_backend_registry};

// TODO: Remove lints.
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::too_many_lines)]
pub(crate) fn handle_command(
    command: Option<&BackendCommand>,
    repository: &mut Repository,
    known_paths: &KnownPaths,
) -> anyhow::Result<ExitCode> {
    let registry = get_backend_registry(known_paths)?;

    match command {
        None | Some(BackendCommand::List) => {
            let backends = repository.database().lock().unwrap().backends()?;
            for (i, backend) in backends.iter().enumerate() {
                println!("{i:2}. {}", backend.name);
            }
        }
        Some(BackendCommand::New { backend_name: name }) => {
            let Some(handler) = registry.get(name.as_str()) else {
                bail!("unknown backend name: {name}");
            };
            let record = BackendRecord {
                id: Uuid::now_v7(),
                enabled: true,
                name: handler.name(),
                config: Config::default(),
            };
            repository.database().lock().unwrap().add_backend(&record)?;
        }
        Some(BackendCommand::Config {
            backend_name: name,
            property_name,
            unset,
            property_value,
        }) => {
            let mut backends = repository.database().lock().unwrap().backends()?;

            let backend = backends
                .iter_mut()
                .find(|backend_record| backend_record.name.contains(name))
                .with_context(|| format!("Could not find field with name {name}"))?;

            let Some(handler) = registry.get(name.as_str()) else {
                bail!("unknown backend name: {name}");
            };

            let schema = handler.config_schema();
            let config = backend.config.align(&schema)?;
            if config != backend.config {
                backend.config = config;
                repository
                    .database()
                    .lock()
                    .unwrap()
                    .update_backend(backend)?;
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
                            |value| value.as_value_string()
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
                repository
                    .database_mut()
                    .get_mut()
                    .unwrap()
                    .update_backend(backend)?;

                return Ok(ExitCode::from(u8::from(!has_value)));
            };

            backend
                .config
                .set(schema_field, property_name, property_value)?;

            repository
                .database_mut()
                .get_mut()
                .unwrap()
                .update_backend(backend)?;
        }
        Some(BackendCommand::Remove { backend_name }) => {
            let backends = repository.database().lock().unwrap().backends()?;

            let backend = backends
                .iter()
                .find(|backend_record| backend_record.name.contains(backend_name))
                .with_context(|| format!("Could not find field with name {backend_name}"))?;

            repository
                .database_mut()
                .get_mut()
                .unwrap()
                .delete_backend(backend.id)?;
        }
    }
    Ok(ExitCode::SUCCESS)
}
