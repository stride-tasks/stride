use std::process::ExitCode;

use anyhow::{Context, bail};
use base64::Engine;
use stride_core::{
    backend::{BackendRecord, Value},
    state::KnownPaths,
};
use stride_flutter_bridge::api::{repository::Repository, settings::ssh_keys};
use url::Url;
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
                config: handler.config_schema().as_config(),
            };
            repository
                .database()
                .lock()
                .unwrap()
                .add_backends(&record)?;
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

            let config = backend.config.align(&handler.config_schema())?;
            if config != backend.config {
                backend.config = config;
                repository
                    .database()
                    .lock()
                    .unwrap()
                    .update_backend(backend)?;
            }

            let Some(property_name) = property_name else {
                for (id, value) in &backend.config.fields {
                    let required = if value.is_some() { "*" } else { "" };
                    println!("{id}: {}{required} = {}", value.as_type_name(), value);
                }
                return Ok(ExitCode::SUCCESS);
            };

            let value = backend
                .config
                .find_field_mut(property_name)
                .with_context(|| format!("Could not find field with name {property_name}"))?;

            let Some(property_value) = property_value else {
                if *unset {
                    assert!(
                        property_value.is_none(),
                        "connot unset with specifying value"
                    );

                    match value {
                        Value::Uuid(value) | Value::SshKey(value) => *value = None,
                        Value::String(value) => *value = None,
                        Value::Bytes(value) | Value::Encryption { value, .. } => {
                            *value = None;
                        }
                        Value::Url(value) => *value = None,
                    }

                    return Ok(ExitCode::SUCCESS);
                }

                let has_value = value.is_some();
                if has_value {
                    println!("{value}");
                }

                repository
                    .database_mut()
                    .get_mut()
                    .unwrap()
                    .update_backend(backend)?;

                return Ok(ExitCode::from(u8::from(!has_value)));
            };

            match value {
                Value::Uuid(value) => {
                    *value = Some(Uuid::parse_str(property_value)?);
                }
                Value::String(value) => {
                    *value = Some(property_value.clone().into_boxed_str());
                }
                Value::Bytes(value) => {
                    *value = Some(property_value.as_bytes().into());
                }
                Value::Url(value) => {
                    *value =
                        Some(Url::parse(property_value).context("config input has invalid URL")?);
                }
                Value::Encryption { mode: _, value } => {
                    let key = base64::engine::general_purpose::URL_SAFE_NO_PAD
                        .decode(property_value)
                        .context("invalid base64 encryption key")?;

                    *value = Some(key);
                }
                Value::SshKey(value) => {
                    let id = Uuid::parse_str(property_value)?;

                    let ssh_keys = ssh_keys()?;
                    let ssh_key = ssh_keys
                        .iter()
                        .find(|ssh_key| ssh_key.uuid() == id)
                        .with_context(|| format!("could not find ssh key with uuid: {id}"))?;

                    *value = Some(ssh_key.uuid());
                }
            }

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
