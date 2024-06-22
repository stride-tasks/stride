use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader, ErrorKind, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

use anyhow::{Context, Result};
use base64::Engine;
use flutter_rust_bridge::frb;
use uuid::Uuid;

use crate::{
    api::{paths::application_support_path, settings::Settings},
    git::known_hosts::{self, HostKeyType, KnownHosts, KnownHostsError},
    task::{Task, TaskBuilder},
};

use git2::{
    CertificateCheckStatus, Cred, ErrorClass, ErrorCode, Mempack, RemoteCallbacks, Repository,
    Signature,
};

use super::{
    filter::{Filter, FilterSelection},
    settings::SshKey,
};

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

impl Task {
    #[frb(sync)]
    pub fn new(description: String) -> Self {
        TaskBuilder::with_description(description)
            .build()
            .expect("All other fields are default initialized")
    }
}

#[frb(opaque)]
pub struct TaskStorage {
    path: PathBuf,
    pending_tasks: Vec<Task>,
    pending_loaded: bool,

    completed_tasks: Vec<Task>,
    completed_loaded: bool,

    deleted_tasks: Vec<Task>,
    deleted_loaded: bool,
}

impl TaskStorage {
    const PENDING_DATA_FILENAME: &'static str = "pending.data";
    const COMPLETE_DATA_FILENAME: &'static str = "complete.data";
    const DELETED_DATA_FILENAME: &'static str = "deleted.data";

    #[frb(sync)]
    pub fn new(path: &str) -> Self {
        let path = Path::new(path).join("repository");
        Self {
            path,
            pending_tasks: Vec::default(),
            pending_loaded: false,
            deleted_tasks: Vec::default(),
            deleted_loaded: false,
            completed_tasks: Vec::default(),
            completed_loaded: false,
        }
    }

    pub fn load(&mut self) -> Result<()> {
        if self.pending_loaded {
            return Ok(());
        }

        let mut tasks = Vec::new();

        let pending_filepath = self.path.join(Self::PENDING_DATA_FILENAME);
        if !pending_filepath.exists() {
            return Ok(());
        }

        let file = File::open(pending_filepath)?;
        let buf = BufReader::new(file);
        for line in buf.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            let task = serde_json::from_str(&line)?;

            tasks.push(task);
        }

        self.pending_tasks = tasks;
        self.pending_loaded = true;
        Ok(())
    }

    pub fn load_deleted(&mut self) -> Result<()> {
        if self.deleted_loaded {
            return Ok(());
        }

        let mut tasks = Vec::new();

        let deleted_filepath = self.path.join(Self::DELETED_DATA_FILENAME);
        if !deleted_filepath.exists() {
            return Ok(());
        }

        let file = File::open(deleted_filepath)?;
        let buf = BufReader::new(file);
        for line in buf.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            let task = serde_json::from_str(&line)?;

            tasks.push(task);
        }

        self.deleted_tasks = tasks;
        self.deleted_loaded = true;
        Ok(())
    }

    pub fn load_completed(&mut self) -> Result<()> {
        if self.completed_loaded {
            return Ok(());
        }

        let mut tasks = Vec::new();

        let completed_filepath = self.path.join(Self::COMPLETE_DATA_FILENAME);
        if !completed_filepath.exists() {
            return Ok(());
        }

        let file = File::open(completed_filepath)?;
        let buf = BufReader::new(file);
        for line in buf.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            let task = serde_json::from_str(&line)?;

            tasks.push(task);
        }

        self.completed_tasks = tasks;
        self.completed_loaded = true;
        Ok(())
    }

    fn save(&mut self) -> Result<()> {
        let pending_path = self.path.join(Self::PENDING_DATA_FILENAME);
        if !pending_path.exists() {
            self.init_repotitory()?;
        }

        let mut content = String::new();
        for task in &self.pending_tasks {
            content += &serde_json::to_string(task)?;
            content.push('\n');
        }

        std::fs::write(pending_path, content)?;
        Ok(())
    }

    pub fn add(&mut self, task: Task) -> Result<()> {
        let pending_path = self.path.join(Self::PENDING_DATA_FILENAME);
        if !pending_path.exists() {
            self.init_repotitory()?;
        }

        let mut file = File::options()
            .append(true)
            .create(true)
            .open(pending_path)?;

        let mut content = serde_json::to_string(&task)?;
        content.push('\n');
        file.write_all(content.as_bytes())?;

        self.pending_tasks.push(task);
        Ok(())
    }

    pub fn task_by_uuid(&mut self, uuid: Uuid) -> Option<Task> {
        self.pending_tasks
            .iter()
            .find(|task| task.uuid == uuid)
            .cloned()
    }

    pub fn tasks_by_description(&mut self, search: String) -> Vec<Task> {
        self.pending_tasks
            .iter()
            .filter(|task| task.description.contains(&search))
            .cloned()
            .collect()
    }

    pub fn tasks_with_filter(&mut self, filter: &Filter) -> Vec<Task> {
        self.pending_tasks
            .iter()
            .filter(|task| task.description.contains(&filter.search))
            .cloned()
            .collect()
    }

    pub fn update(&mut self, task: Task) -> Result<bool> {
        let current = self
            .pending_tasks
            .iter_mut()
            .find(|element| element.uuid == task.uuid);
        let Some(current) = current else {
            return Ok(false);
        };
        *current = task;

        self.save()?;
        Ok(true)
    }

    pub fn complete(&mut self, uuid: Uuid) -> Result<()> {
        let index = self
            .pending_tasks
            .iter()
            .position(|task| task.uuid == uuid)
            .with_context(|| format!("No task found with uuid: {uuid}"))?;

        let task = self.pending_tasks.remove(index);
        self.save()?;

        let completed_filepath = self.path.join(Self::COMPLETE_DATA_FILENAME);
        let mut file = File::options()
            .append(true)
            .create(true)
            .open(completed_filepath)?;

        let mut content = serde_json::to_string(&task)?;
        content.push('\n');
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn delete(&mut self, uuid: Uuid) -> Result<()> {
        let index = self
            .pending_tasks
            .iter()
            .position(|task| task.uuid == uuid)
            .with_context(|| format!("No task found with uuid: {uuid}"))?;

        let task = self.pending_tasks.remove(index);
        self.save()?;

        let delete_filepath = self.path.join(Self::DELETED_DATA_FILENAME);
        let mut file = File::options()
            .append(true)
            .create(true)
            .open(delete_filepath)?;

        let mut content = serde_json::to_string(&task)?;
        content.push('\n');
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn tasks(&self) -> Vec<Task> {
        self.pending_tasks.clone()
    }
    pub fn deleted_tasks(&self) -> Vec<Task> {
        self.deleted_tasks.clone()
    }
    pub fn completed_tasks(&self) -> Vec<Task> {
        self.deleted_tasks.clone()
    }

    pub fn sync(&mut self) -> Result<(), ConnectionError> {
        if self.path.exists() {
            self.add_and_commit().unwrap();
            Ok(())
        } else {
            self.clone_repository()
        }
    }

    pub fn clear_contents(&mut self) {
        self.pending_tasks.clear();
        std::fs::remove_dir_all(&self.path).unwrap();
    }

    pub fn clone_repository(&mut self) -> Result<(), ConnectionError> {
        let settings = Settings::get();

        let Some(ssh_key_uuid) = &settings.repository.ssh_key_uuid else {
            return Err(ConnectionError::NoSshKeysProvided);
        };
        let ssh_key = settings
            .ssh_key(ssh_key_uuid)
            .expect("there should be a key with the specified uuid");

        let mut callbacks = RemoteCallbacks::new();
        let callback_error = with_authentication(
            ssh_key.clone(),
            settings.known_hosts.clone(),
            &mut callbacks,
        );

        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fo);

        let connection = builder.clone(&settings.repository.origin, &self.path);

        if let Err(error) = &connection {
            return match error.class() {
                ErrorClass::Ssh => Err(ConnectionError::Authentication {
                    message: error.message().to_owned(),
                }),
                ErrorClass::Net => Err(ConnectionError::Network {
                    message: error.message().to_owned(),
                }),
                ErrorClass::Callback => {
                    let mut callback_error = callback_error.borrow_mut();
                    if let Some(callback_error) = callback_error.take() {
                        return Err(callback_error.clone());
                    }
                    Err(ConnectionError::Other {
                        message: error.message().to_owned(),
                    })
                }
                _ => Err(ConnectionError::Other {
                    message: error.message().to_owned(),
                }),
            };
        }

        log::info!(
            "Repository {} cloned successfully!",
            settings.repository.origin
        );

        self.pending_loaded = false;
        self.deleted_loaded = false;
        self.completed_loaded = false;

        Ok(())
    }

    pub fn init_repotitory(&self) -> anyhow::Result<()> {
        let settings = Settings::get();

        let repository = Repository::init(&self.path)?;

        let mut index = repository.index()?;

        index.add_all(["."], git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let tree = index.write_tree()?;
        let tree = repository.find_tree(tree)?;

        let author = Signature::now(&settings.repository.author, &settings.repository.email)?;

        let commit = repository.commit(None, &author, &author, "Initial Commit", &tree, &[])?;
        let commit = repository.find_commit(commit)?;

        let branch = repository.branch("main", &commit, true)?;
        let mut branch_ref = branch.into_reference();
        branch_ref.set_target(commit.id(), "update it")?;
        let branch_ref_name = branch_ref.name().unwrap();
        repository.set_head(branch_ref_name)?;

        Result::Ok(())
    }

    pub fn add_and_commit(&self) -> anyhow::Result<()> {
        let settings = Settings::get();

        let repository = Repository::open(&self.path)?;

        if repository.statuses(None)?.is_empty() {
            println!("Skipping sync, no changes done");
            return Ok(());
        }

        let mut index = repository.index()?;

        index.add_all(["."], git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let tree = index.write_tree()?;
        let tree = repository.find_tree(tree)?;

        let author = Signature::now(&settings.repository.author, &settings.repository.email)?;

        let parent_commit = repository.head()?.peel_to_commit()?;

        let commit = repository.commit(
            Some("HEAD"),
            &author,
            &author,
            "initial",
            &tree,
            &[&parent_commit],
        )?;
        let commit = repository.find_commit(commit)?;

        let branch = repository.find_branch("main", git2::BranchType::Local)?;
        let mut branch_ref = branch.into_reference();
        branch_ref.set_target(commit.id(), "update it")?;
        let branch_ref_name = branch_ref.name().unwrap();
        repository.set_head(branch_ref_name)?;

        let Some(ssh_key_uuid) = &settings.repository.ssh_key_uuid else {
            return Err(ConnectionError::NoSshKeysProvided.into());
        };
        let ssh_key = settings
            .ssh_key(ssh_key_uuid)
            .expect("there should be a key with the specified uuid");
        let mut callbacks = RemoteCallbacks::new();
        let callback_error = with_authentication(
            ssh_key.clone(),
            settings.known_hosts.clone(),
            &mut callbacks,
        );
        callbacks.push_update_reference(|name, status| {
            println!("{name}: {status:?}");
            Result::Ok(())
        });

        let _ = repository.remote("origin", &settings.repository.origin);
        let _ = repository.remote_set_url("origin", &settings.repository.origin);

        let mut origin = repository.find_remote("origin")?;
        let connection = origin.connect_auth(git2::Direction::Push, Some(callbacks), None);

        if let Err(error) = &connection {
            return match error.class() {
                ErrorClass::Ssh => Err(ConnectionError::Authentication {
                    message: error.message().to_owned(),
                }
                .into()),
                ErrorClass::Net => Err(ConnectionError::Network {
                    message: error.message().to_owned(),
                }
                .into()),
                ErrorClass::Callback => {
                    let mut callback_error = callback_error.borrow_mut();
                    if let Some(callback_error) = callback_error.take() {
                        return Err(callback_error.clone().into());
                    }
                    Err(ConnectionError::Other {
                        message: error.message().to_owned(),
                    }
                    .into())
                }
                _ => Err(ConnectionError::Other {
                    message: error.message().to_owned(),
                }
                .into()),
            };
        }
        let connection = connection.unwrap().remote().push(&[branch_ref_name], None);

        if let Err(error) = &connection {
            return match error.class() {
                ErrorClass::Ssh => Err(ConnectionError::Authentication {
                    message: error.message().to_owned(),
                }
                .into()),
                ErrorClass::Net => Err(ConnectionError::Network {
                    message: error.message().to_owned(),
                }
                .into()),
                ErrorClass::Callback => {
                    let mut callback_error = callback_error.borrow_mut();
                    if let Some(callback_error) = callback_error.take() {
                        return Err(callback_error.clone().into());
                    }
                    Err(ConnectionError::Other {
                        message: error.message().to_owned(),
                    }
                    .into())
                }
                _ => Err(ConnectionError::Other {
                    message: error.message().to_owned(),
                }
                .into()),
            };
        }

        log::info!("Task sync finished!");

        Result::Ok(())
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ConnectionError {
    #[error("network error: {message}")]
    Network { message: String },

    #[error("no ssh keys are provided")]
    NoSshKeysProvided,

    #[error("ssh authentication error: {message}")]
    Authentication { message: String },

    #[error("unknown host error: {hostname} with {key_type} {host_key}")]
    UnknownHost {
        hostname: String,
        key_type: HostKeyType,
        host_key: String,
    },

    #[error("{hostname} remote host key is not available")]
    MissingHostKey { hostname: String },

    #[error("unknown remote key type")]
    UnknownKeyType,

    #[error("mismatched host key")]
    MissmatchRemoteKey { expected: String, actual: String },

    #[error("{message}")]
    Other { message: String },
}

pub fn test_connection(
    url: String,
    public_key: String,
    private_key: String,
) -> Result<(), ConnectionError> {
    let settings = Settings::get();

    let Some(ssh_key_uuid) = &settings.repository.ssh_key_uuid else {
        return Err(ConnectionError::NoSshKeysProvided);
    };
    let ssh_key = settings
        .ssh_key(ssh_key_uuid)
        .expect("there should be a key with the specified uuid");

    let mut callbacks = RemoteCallbacks::new();
    let callback_error = with_authentication(
        ssh_key.clone(),
        settings.known_hosts.clone(),
        &mut callbacks,
    );

    let mut remote =
        git2::Remote::create_detached(url).map_err(|error| ConnectionError::Other {
            message: error.to_string(),
        })?;
    let connection = remote
        .connect_auth(git2::Direction::Fetch, Some(callbacks), None)
        .map(|_| ());

    if let Err(error) = &connection {
        return match error.class() {
            ErrorClass::Ssh => Err(ConnectionError::Authentication {
                message: error.message().to_owned(),
            }),
            ErrorClass::Net => Err(ConnectionError::Network {
                message: error.message().to_owned(),
            }),
            ErrorClass::Callback => {
                let mut callback_error = callback_error.borrow_mut();
                if let Some(callback_error) = callback_error.take() {
                    return Err(callback_error.clone());
                }
                Err(ConnectionError::Other {
                    message: error.message().to_owned(),
                })
            }
            _ => Err(ConnectionError::Other {
                message: error.message().to_owned(),
            }),
        };
    }

    Result::Ok(())
}

fn with_authentication(
    ssh_key: SshKey,
    known_hosts: KnownHosts,
    callbacks: &mut RemoteCallbacks<'_>,
) -> Rc<RefCell<Option<ConnectionError>>> {
    let mut tried_ssh = false;

    let mut error = Rc::<RefCell<Option<ConnectionError>>>::default();

    // See: https://github.com/rust-lang/git2-rs/issues/347
    callbacks.credentials(move |_url, username_from_url, allowed_types| {
        if tried_ssh {
            log::error!("Failed to authenticate with credentials");
            return Err(git2::Error::new(
                ErrorCode::Auth,
                ErrorClass::Ssh,
                "Failed to authenticate with credentials",
            ));
        }
        let Some(username) = username_from_url else {
            return Err(git2::Error::new(
                ErrorCode::Auth,
                ErrorClass::Ssh,
                "No username provide in the url",
            ));
        };

        tried_ssh = true;

        Cred::ssh_key_from_memory(
            username_from_url.unwrap(),
            Some(&ssh_key.public),
            &ssh_key.private,
            None,
        )
    });

    let certificate_error = error.clone();
    callbacks.certificate_check(move |cert, hostname| {
        let Some(cert_host_key) = cert.as_hostkey() else {
            return Result::Ok(CertificateCheckStatus::CertificatePassthrough);
        };
        let Some(host_key_type) = cert_host_key.hostkey_type() else {
            *certificate_error.borrow_mut() = Some(ConnectionError::MissingHostKey {
                hostname: hostname.to_owned(),
            });
            return Err(git2::Error::new(
                ErrorCode::Certificate,
                ErrorClass::Callback,
                "remote host key is not available",
            ));
        };
        let host_key = cert_host_key.hostkey().unwrap();
        let host_key = base64::engine::general_purpose::STANDARD.encode(host_key);

        let Result::Ok(host_key_type) = HostKeyType::try_from(host_key_type) else {
            *certificate_error.borrow_mut() = Some(ConnectionError::UnknownKeyType);
            return Err(git2::Error::new(
                ErrorCode::Certificate,
                ErrorClass::Callback,
                "unknown remote key type",
            ));
        };

        let Some(host) = known_hosts.host(hostname, host_key_type) else {
            *certificate_error.borrow_mut() = Some(ConnectionError::UnknownHost {
                hostname: hostname.to_owned(),
                key_type: host_key_type,
                host_key,
            });
            return Err(git2::Error::new(
                ErrorCode::Certificate,
                ErrorClass::Callback,
                "unknown host",
            ));
        };

        if host.remote_host_key != host_key {
            *certificate_error.borrow_mut() = Some(ConnectionError::MissmatchRemoteKey {
                expected: host.remote_host_key.to_owned(),
                actual: host_key,
            });
            return Err(git2::Error::new(
                ErrorCode::Certificate,
                ErrorClass::Callback,
                "mismatched host key",
            ));
        }

        Result::Ok(CertificateCheckStatus::CertificateOk)
    });

    // let connection = f(callbacks);

    // if let Err(error) = &connection {
    //     return match error.class() {
    //         ErrorClass::Ssh => Err(ConnectionError::Authentication {
    //             message: error.message().to_owned(),
    //         }),
    //         ErrorClass::Net => Err(ConnectionError::Network {
    //             message: error.message().to_owned(),
    //         }),
    //         ErrorClass::Callback => {
    //             if let Some(certificate_error) = &error {
    //                 return Err(certificate_error.clone());
    //             }
    //             Err(ConnectionError::Other {
    //                 message: error.message().to_owned(),
    //             })
    //         }
    //         _ => Err(ConnectionError::Other {
    //             message: error.message().to_owned(),
    //         }),
    //     };
    // }

    error
}

// fn clone(url: String) -> Result<(), ConnectionError> {
//     let settings = Settings::load().unwrap();
//     let mut callbacks = RemoteCallbacks::new();
//     let error = with_authentication(
//         settings.keys[0].public.clone(),
//         settings.keys[0].private.clone(),
//         &mut callbacks,
//     );

//     Result::Ok(())
// }
