use std::path::PathBuf;

use stride_core::config::{Field, Schema, Value};

use crate::config::FromSchema;

use super::{encryption_key::EncryptionKey, ssh_key::SshKey};

#[derive(Debug, Clone)]
pub struct GitConfig {
    pub root_path: PathBuf,

    pub author: Box<str>,
    pub email: Box<str>,
    pub branch: Box<str>,
    pub origin: Box<str>,
    pub encryption_key: EncryptionKey,
    pub ssh_key: SshKey,
}

impl FromSchema for GitConfig {
    fn from_schema(
        schema: &Schema,
        root_path: &std::path::Path,
    ) -> Result<Self, stride_core::config::SchemaError> {
        Ok(Self {
            root_path: root_path.to_path_buf(),
            author: schema.string_value("author")?.into(),
            email: schema.string_value("email")?.into(),
            branch: schema.string_value("branch")?.into(),
            origin: schema.string_value("origin")?.into(),
            encryption_key: EncryptionKey {
                key: schema.string_value("encryption_key")?.into(),
            },
            ssh_key: todo!(),
        })
    }
    fn default_schema() -> Schema {
        Schema {
            title: "Git".into(),
            fields: vec![
                Field {
                    id: "author".into(),
                    name: "Author".into(),
                    value: Value::String(Some("Stride".into())),
                },
                Field {
                    id: "email".into(),
                    name: "email".into(),
                    value: Value::String(Some("noreply.stride.tasks@gmail.com".into())),
                },
                Field {
                    id: "branch".into(),
                    name: "branch".into(),
                    value: Value::String(Some("main".into())),
                },
                Field {
                    id: "encryption_key".into(),
                    name: "Encryption Key".into(),
                    value: Value::String(None),
                },
                Field {
                    id: "ssh_key_uuid".into(),
                    name: "SSH Key".into(),
                    value: Value::Uuid(None),
                },
            ]
            .into(),
        }
    }
}

impl GitConfig {
    const TASK_DIR: &str = "tasks";

    #[must_use]
    pub fn repository_path(&self) -> PathBuf {
        self.root_path.join("source")
    }

    #[must_use]
    pub fn tasks_path(&self) -> PathBuf {
        self.repository_path().join(Self::TASK_DIR)
    }

    #[must_use]
    pub fn keys_filepath(&self) -> PathBuf {
        self.tasks_path().join("keys")
    }
}
