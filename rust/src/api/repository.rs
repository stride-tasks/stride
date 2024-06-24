use core::task;
use std::{
    cell::RefCell,
    collections::HashSet,
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
    task::{Task, TaskBuilder, TaskStatus},
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

struct Storage {
    loaded: bool,
    tasks: Vec<Task>,
    path: PathBuf,
    kind: TaskStatus,
}

impl Storage {
    fn new(path: PathBuf, kind: TaskStatus) -> Self {
        Self {
            loaded: false,
            tasks: Vec::new(),
            path,
            kind,
        }
    }

    fn load(&mut self) -> Result<()> {
        if self.loaded {
            return Ok(());
        }

        if !self.path.exists() {
            return Ok(());
        }

        let file = File::open(&self.path)?;
        let buf = BufReader::new(file);
        let mut tasks = Vec::new();
        for line in buf.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            let mut task: Task = serde_json::from_str(&line)?;
            task.status = self.kind;

            tasks.push(task);
        }

        self.tasks = tasks;
        self.loaded = true;
        Ok(())
    }

    fn append(&mut self, mut task: Task) -> Result<()> {
        task.status = self.kind;

        let mut file = File::options().append(true).create(true).open(&self.path)?;

        let mut content = serde_json::to_string(&task)?;
        content.push('\n');
        file.write_all(content.as_bytes())?;
        self.tasks.push(task);
        Ok(())
    }

    fn save(&mut self) -> Result<()> {
        let mut content = String::new();
        for task in &self.tasks {
            content += &serde_json::to_string(task)?;
            content.push('\n');
        }

        std::fs::write(&self.path, content)?;
        Ok(())
    }

    fn get_by_id(&mut self, uuid: &Uuid) -> Result<Option<&Task>> {
        self.load()?;
        Ok(self.tasks.iter().find(|task| &task.uuid == uuid))
    }

    fn get_index(&mut self, uuid: &Uuid) -> Result<Option<usize>> {
        self.load()?;
        Ok(self.tasks.iter().position(|task| &task.uuid == uuid))
    }

    fn filter(&mut self, filter: &Filter, result: &mut Vec<Task>) -> Result<()> {
        if !filter.status.contains(&self.kind) {
            return Ok(());
        }

        self.load()?;
        for task in self
            .tasks
            .iter()
            .filter(|task| task.description.contains(&filter.search))
        {
            result.push(task.clone());
        }

        Ok(())
    }

    fn update(&mut self, task: &Task) -> Result<bool> {
        if task.status != self.kind {
            return Ok(false);
        }

        self.load()?;
        let current = self
            .tasks
            .iter_mut()
            .find(|element| element.uuid == task.uuid);
        let Some(current) = current else {
            return Ok(false);
        };
        *current = task.clone();

        self.save()?;
        Ok(true)
    }

    fn remove(&mut self, uuid: &Uuid) -> Result<Option<Task>> {
        self.load()?;
        let index = self
            .tasks
            .iter_mut()
            .position(|element| &element.uuid == uuid);
        let Some(index) = index else {
            return Ok(None);
        };

        let task = self.tasks.remove(index);
        self.save()?;
        Ok(Some(task))
    }

    fn clear(&mut self) -> Result<()> {
        self.loaded = true;
        self.tasks.clear();
        self.save()?;
        Ok(())
    }
    fn unload(&mut self) {
        self.loaded = true;
        self.tasks.clear();
    }
}

#[frb(opaque)]
pub struct TaskStorage {
    path: PathBuf,

    pending: Storage,
    completed: Storage,
    deleted: Storage,
    waiting: Storage,
    recurring: Storage,
}

impl TaskStorage {
    const PENDING_DATA_FILENAME: &'static str = "pending.data";
    const COMPLETE_DATA_FILENAME: &'static str = "complete.data";
    const DELETED_DATA_FILENAME: &'static str = "deleted.data";
    const WAITING_DATA_FILENAME: &'static str = "waiting.data";
    const RECURRING_DATA_FILENAME: &'static str = "recurring.data";

    #[frb(sync)]
    pub fn new(path: &str) -> Self {
        let path = Path::new(path).join("repository");
        Self {
            pending: Storage::new(path.join(Self::PENDING_DATA_FILENAME), TaskStatus::Pending),
            completed: Storage::new(
                path.join(Self::COMPLETE_DATA_FILENAME),
                TaskStatus::Complete,
            ),
            deleted: Storage::new(path.join(Self::DELETED_DATA_FILENAME), TaskStatus::Deleted),
            waiting: Storage::new(path.join(Self::WAITING_DATA_FILENAME), TaskStatus::Waiting),
            recurring: Storage::new(
                path.join(Self::RECURRING_DATA_FILENAME),
                TaskStatus::Recurring,
            ),
            path,
        }
    }

    fn storage_mut(&mut self) -> [&mut Storage; 5] {
        [
            &mut self.pending,
            &mut self.waiting,
            &mut self.recurring,
            &mut self.deleted,
            &mut self.completed,
        ]
    }

    pub fn unload(&mut self) {
        for storage in self.storage_mut() {
            storage.unload();
        }
    }

    pub fn add(&mut self, task: Task) -> Result<()> {
        if !self.path.exists() {
            self.init_repotitory()?;
        }

        self.pending.append(task)?;
        Ok(())
    }

    pub fn task_by_uuid(&mut self, uuid: &Uuid) -> Result<Option<Task>> {
        for storage in self.storage_mut() {
            let task = storage.get_by_id(uuid)?;

            if let Some(task) = task {
                return Ok(Some(task.clone()));
            }
        }
        Ok(None)
    }

    pub fn tasks_with_filter(&mut self, filter: &Filter) -> anyhow::Result<Vec<Task>> {
        let mut tasks = Vec::new();
        for storage in self.storage_mut() {
            storage.filter(filter, &mut tasks)?;
        }

        Ok(tasks)
    }

    fn remove(&mut self, uuid: &Uuid) -> Result<Option<Task>> {
        for storage in self.storage_mut() {
            if let Some(task) = storage.remove(uuid)? {
                return Ok(Some(task));
            }
        }
        Ok(None)
    }

    pub fn update(&mut self, task: Task) -> Result<bool> {
        for storage in self.storage_mut() {
            if storage.update(&task)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn change_category(&mut self, task: Task, status: TaskStatus) -> Result<bool> {
        if task.status == status {
            return Ok(true);
        }

        let mut found_task = None;
        for storage in self.storage_mut() {
            if task.status != storage.kind {
                continue;
            }

            storage.load()?;
            let index = storage
                .tasks
                .iter()
                .position(|element| element.uuid == task.uuid);
            let Some(index) = index else {
                break;
            };

            found_task = Some(storage.tasks.remove(index));

            storage.save()?;
        }

        let mut found_task =
            found_task.with_context(|| format!("No task found with uuid: {}", task.uuid))?;

        found_task.status = status;

        self.storage_mut()[status as usize].append(found_task)?;
        Ok(false)
    }

    pub fn remove_task(&mut self, task: &Task) -> Result<bool> {
        let mut found_task = None;
        for storage in self.storage_mut() {
            if task.status != storage.kind {
                continue;
            }

            storage.load()?;
            let index = storage
                .tasks
                .iter()
                .position(|element| element.uuid == task.uuid);
            let Some(index) = index else {
                break;
            };

            found_task = Some(storage.tasks.remove(index));

            storage.save()?;
        }

        let mut _found_task =
            found_task.with_context(|| format!("No task found with uuid: {}", task.uuid))?;
        Ok(false)
    }

    pub fn tasks(&mut self) -> Result<Vec<Task>> {
        self.tasks_with_filter(&Filter {
            name: "default".to_owned(),
            status: HashSet::from_iter([TaskStatus::Pending]),
            uuid: Uuid::new_v4(),
            search: String::new(),
        })
    }

    pub fn sync(&mut self) -> Result<(), ConnectionError> {
        if self.path.exists() {
            self.add_and_commit().unwrap();
            Ok(())
        } else {
            self.clone_repository()
        }
    }

    pub fn clear(&mut self) -> Result<()> {
        for storage in self.storage_mut() {
            storage.clear()?;
        }
        std::fs::remove_dir_all(&self.path)?;
        Ok(())
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

        self.unload();

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
        let connection = connection?.remote().push(&[branch_ref_name], None);

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

    error
}
