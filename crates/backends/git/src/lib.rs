//! Stride's backend implementations.

#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use base64::{DecodeError, Engine};
use config::GitConfig;
use git2::{
    AnnotatedCommit, Branch, CertificateCheckStatus, Cred, ErrorClass, ErrorCode, FetchOptions,
    Oid, RebaseOptions, RemoteCallbacks, Repository, Signature, build::CheckoutBuilder,
};
use known_hosts::{Host, HostKeyType, KnownHosts};
use serialization::task_to_data;
use ssh_key::SshKey;
use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use stride_backend::{Backend, BackendHandler};
use stride_core::{
    event::TaskQuery,
    task::{Task, TaskStatus},
};
use stride_crypto::crypter::Crypter;
use stride_database::Database;
use uuid::Uuid;

pub mod error;

pub use error::{Error, Result};

pub(crate) fn base64_encode<T: AsRef<[u8]>>(input: T) -> String {
    fn inner(input: &[u8]) -> String {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input)
    }
    inner(input.as_ref())
}

pub(crate) fn base64_decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
    fn inner(input: &[u8]) -> Result<Vec<u8>, DecodeError> {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(input)
    }
    inner(input.as_ref())
}

pub(crate) trait ToBase64 {
    fn to_base64(&self) -> String;
}

impl ToBase64 for Uuid {
    fn to_base64(&self) -> String {
        base64_encode(self.as_bytes())
    }
}

mod key_store;
mod serialization;

/// flutter_rust_bridge:ignore
pub mod config;

pub mod encryption_key;
pub mod known_hosts;
pub mod ssh_key;

use key_store::KeyStore;

use crate::config::Handler;

pub(crate) const IV_LEN: usize = 12;

pub(crate) fn generate_iv() -> [u8; IV_LEN] {
    let mut iv = [0u8; IV_LEN];
    getrandom::fill(&mut iv).unwrap();
    iv
}

pub(crate) struct TaskDiff {
    path: PathBuf,
    adding: bool,
    // TODO: content no longer has to be a String, it can be a &[u8]
    content: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
struct DecryptedTask {
    task: Task,
    #[serde(skip, default = "generate_iv")]
    iv: [u8; IV_LEN],
}

struct Storage {
    loaded: bool,
    tasks: Vec<DecryptedTask>,
    path: PathBuf,
    kind: TaskStatus,
    key_store: Arc<KeyStore>,
}

impl Storage {
    fn new(path: PathBuf, kind: TaskStatus, key_store: Arc<KeyStore>) -> Self {
        Self {
            loaded: false,
            tasks: Vec::new(),
            path,
            kind,
            key_store,
        }
    }

    fn load(&mut self) -> Result<()> {
        if self.loaded {
            return Ok(());
        }

        if !self.path.exists() {
            return Ok(());
        }

        let file: File = File::open(&self.path)?;
        let buf = BufReader::new(file);
        let mut tasks = Vec::new();
        for line in buf.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            let (iv, mut task) = self.key_store.decrypt(self.kind, &line)?;
            task.status = self.kind;

            tasks.push(DecryptedTask { task, iv });
        }

        self.tasks = tasks;
        self.loaded = true;
        Ok(())
    }

    fn append(&mut self, mut task: Task) -> Result<()> {
        task.status = self.kind;

        let mut file = File::options().append(true).create(true).open(&self.path)?;

        if self.key_store.has_key_for(self.kind)? {
            let (iv, mut content) = self.key_store.encrypt(&task, None)?;
            content.push('\n');
            file.write_all(content.as_bytes())?;
            self.tasks.push(DecryptedTask { task, iv });
        } else {
            let mut content = task_to_data(&task);
            content.push(b'\n');
            file.write_all(&content)?;

            drop(file);

            let iv = generate_iv();
            self.tasks.push(DecryptedTask { task, iv });
            self.key_store.save()?;
            self.save()?;
        }

        Ok(())
    }

    fn save(&mut self) -> Result<()> {
        let mut content = String::new();
        for DecryptedTask { task, iv } in &self.tasks {
            let (_, data) = self.key_store.encrypt(task, Some(*iv))?;
            content += &data;
            content.push('\n');
        }

        std::fs::write(&self.path, content)?;
        Ok(())
    }

    fn get_by_id(&mut self, uuid: &Uuid) -> Result<Option<&Task>> {
        self.load()?;
        Ok(self
            .tasks
            .iter()
            .find(|task| &task.task.uuid == uuid)
            .map(|et| &et.task))
    }

    #[allow(unused)]
    fn get_index(&mut self, uuid: &Uuid) -> Result<Option<usize>> {
        self.load()?;
        Ok(self.tasks.iter().position(|task| &task.task.uuid == uuid))
    }

    // fn filter(&mut self, filter: &Filter, result: &mut Vec<Task>) -> Result<()> {
    //     if !filter.status.contains(&self.kind) {
    //         return Ok(());
    //     }

    //     let search = filter.search.to_lowercase();

    //     self.load()?;
    //     for DecryptedTask { task, .. } in self
    //         .tasks
    //         .iter()
    //         .filter(|DecryptedTask { task, .. }| task.title.to_lowercase().contains(&search))
    //     {
    //         result.push(task.clone());
    //     }

    //     Ok(())
    // }
    fn query(&mut self, query: &TaskQuery, result: &mut Vec<Task>) -> Result<()> {
        match query {
            TaskQuery::Uuid { uuid } => {
                if let Some(task) = self.get_by_id(uuid)? {
                    result.push(task.clone());
                }
            }
            TaskQuery::Title {
                title,
                status,
                limit,
            } => {
                if !status.is_empty() && !status.contains(&self.kind) {
                    return Ok(());
                }

                let search = title.to_lowercase();

                self.load()?;
                for DecryptedTask { task, .. } in
                    self.tasks.iter().filter(|DecryptedTask { task, .. }| {
                        task.title.to_lowercase().contains(&search)
                    })
                {
                    #[allow(clippy::cast_possible_truncation)]
                    if Some(result.len() as u32) == *limit {
                        break;
                    }
                    result.push(task.clone());
                }
            }
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
            .find(|DecryptedTask { task: element, .. }| element.uuid == task.uuid);

        let Some(current) = current else {
            return Ok(false);
        };

        if current.task == *task {
            return Ok(false);
        }

        current.task = task.clone();
        current.iv = generate_iv();

        self.save()?;
        Ok(true)
    }

    fn remove(&mut self, uuid: &Uuid) -> Result<Option<Task>> {
        let index = self.get_index(uuid)?;
        let Some(index) = index else {
            return Ok(None);
        };

        let DecryptedTask { task, .. } = self.tasks.remove(index);
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
        self.loaded = false;
        self.tasks.clear();
    }
}

#[allow(missing_debug_implementations)]
pub struct GitBackend {
    config: GitConfig,

    tasks_path: PathBuf,
    key_store: Arc<KeyStore>,

    pending: Storage,
    complete: Storage,
    deleted: Storage,
    waiting: Storage,
    recurring: Storage,
}

impl GitBackend {
    const PENDING_DATA_FILENAME: &'static str = "pending";
    const COMPLETE_DATA_FILENAME: &'static str = "complete";
    const DELETED_DATA_FILENAME: &'static str = "deleted";
    const WAITING_DATA_FILENAME: &'static str = "waiting";
    const RECURRING_DATA_FILENAME: &'static str = "recurring";

    pub fn new(config: GitConfig) -> Result<Self> {
        let repository_path = config.repository_path();
        let keys_filepath = config.keys_filepath();

        std::fs::create_dir_all(&repository_path)?;

        let key = base64_decode(&config.encryption_key.key)?;
        let crypter = Arc::new(Crypter::new(key.try_into().unwrap()));
        let key_store = Arc::new(KeyStore::new(&keys_filepath, crypter));

        let tasks_path = config.tasks_path();

        Ok(Self {
            config,
            pending: Storage::new(
                tasks_path.join(Self::PENDING_DATA_FILENAME),
                TaskStatus::Pending,
                key_store.clone(),
            ),
            complete: Storage::new(
                tasks_path.join(Self::COMPLETE_DATA_FILENAME),
                TaskStatus::Complete,
                key_store.clone(),
            ),
            deleted: Storage::new(
                tasks_path.join(Self::DELETED_DATA_FILENAME),
                TaskStatus::Deleted,
                key_store.clone(),
            ),
            waiting: Storage::new(
                tasks_path.join(Self::WAITING_DATA_FILENAME),
                TaskStatus::Waiting,
                key_store.clone(),
            ),
            recurring: Storage::new(
                tasks_path.join(Self::RECURRING_DATA_FILENAME),
                TaskStatus::Recurring,
                key_store.clone(),
            ),
            tasks_path,
            key_store,
        })
    }

    fn storage_mut(&mut self) -> [&mut Storage; 5] {
        [
            &mut self.pending,
            &mut self.waiting,
            &mut self.recurring,
            &mut self.deleted,
            &mut self.complete,
        ]
    }

    pub(crate) fn update2(&mut self, task: &Task) -> Result<bool> {
        let mut updated = false;
        for storage in self.storage_mut() {
            if storage.update(task)? {
                updated = true;
                break;
            }
        }
        Ok(updated)
    }

    pub(crate) fn remove_task2(&mut self, task: &Task) -> Result<Option<Task>> {
        let mut found_task = None;
        for storage in self.storage_mut() {
            if task.status != storage.kind {
                continue;
            }

            let index = storage.get_index(&task.uuid)?;
            let Some(index) = index else {
                break;
            };

            found_task = Some(storage.tasks.remove(index));

            storage.save()?;
        }
        Ok(found_task.map(|DecryptedTask { task, .. }| task))
    }

    fn change_category(&mut self, task: &Task, status: TaskStatus) -> Result<bool> {
        if task.status == status {
            return Ok(false);
        }

        let mut found_task = None;
        for storage in self.storage_mut() {
            if task.status != storage.kind {
                continue;
            }

            let index = storage.get_index(&task.uuid)?;
            let Some(index) = index else {
                break;
            };

            found_task = Some(storage.tasks.remove(index));

            storage.save()?;
        }

        let Some(mut found_task) = found_task else {
            return Ok(false);
        };

        found_task.task.status = status;
        let transition = match status {
            TaskStatus::Pending => "PEND",
            TaskStatus::Waiting => "WAIT",
            TaskStatus::Recurring => "RECUR",
            TaskStatus::Deleted => "DELETE",
            TaskStatus::Complete => "DONE",
        };

        let message = format!("${transition} {}", found_task.task.uuid.to_base64());
        self.storage_mut()[status as usize].append(found_task.task)?;
        self.add_and_commit(&message)?;
        Ok(true)
    }

    pub fn clone_repository(&mut self) -> Result<()> {
        let mut callbacks = RemoteCallbacks::new();
        let callback_error = with_authentication(self.config.ssh_key.clone(), &mut callbacks);

        let mut fo = FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fo);
        builder.branch(&self.config.branch);

        let connection = builder.clone(&self.config.origin, &self.config.repository_path());

        if let Some(callback_error) = callback_error.borrow_mut().take() {
            return Err(callback_error);
        }

        if let Err(err) = connection {
            // Empty repository case:
            if (err.class(), err.code()) != (ErrorClass::Reference, ErrorCode::NotFound) {
                return Err(err.into());
            }

            let repository = Repository::init(self.config.repository_path())?;
            self.init_repotitory(&repository)?;
            self.push(&repository, true)?;
        }

        log::info!("Repository {} cloned successfully!", &self.config.origin);

        self.unload();

        if !self.tasks_path.exists() {
            std::fs::create_dir(&self.tasks_path)
                .expect("creating the tasks directory should not fail");
        }

        Ok(())
    }

    pub fn force_hard_reset(&mut self, commit: Oid) -> Result<()> {
        let repository = Repository::open(self.config.repository_path())?;
        let commit = repository.find_commit(commit)?;

        let branch = repository.find_branch(&self.config.branch, git2::BranchType::Local)?;

        let mut reference = branch.into_reference();

        reference.set_target(
            commit.id(),
            &format!("Force hard reset to commit: {}", commit.id()),
        )?;

        let mut checkout = CheckoutBuilder::new();
        checkout.force();
        repository.reset(
            commit.as_object(),
            git2::ResetType::Hard,
            Some(&mut checkout),
        )?;

        Ok(())
    }

    // pub fn checkout(&mut self) -> Result<()> {
    //     self.init_repository_if_needed()?;

    //     self.sync()?;
    //     self.unload();

    //     let git_repository = Repository::open(&self.config.repository_path())?;
    //     let branch = git_repository.find_branch(&self.config.branch, git2::BranchType::Local)?;
    //     let reference = branch.into_reference();
    //     let tree = reference.peel_to_tree()?;
    //     git_repository.checkout_tree(tree.as_object(), None)?;

    //     let name = reference
    //         .name()
    //         .expect("invalid UTF-8 reference name of branch");
    //     git_repository.set_head(name)?;

    //     if !self.tasks_path.exists() {
    //         std::fs::create_dir(&self.tasks_path)?;
    //     }

    //     Ok(())
    // }

    pub(crate) fn init_repotitory<'a>(&self, repository: &'a Repository) -> Result<Branch<'a>> {
        let mut index = repository.index()?;

        let tree = index.write_tree()?;
        let tree = repository.find_tree(tree)?;

        let author = Signature::now(&self.config.author, &self.config.email)?;

        let commit = repository.commit(None, &author, &author, "Initial Commit", &tree, &[])?;
        let commit = repository.find_commit(commit)?;

        let branch = repository.branch(&self.config.branch, &commit, true)?;
        let mut branch_ref = branch.into_reference();
        branch_ref.set_target(commit.id(), "update it")?;
        let branch_ref_name = branch_ref.name().unwrap();
        repository.set_head(branch_ref_name)?;

        let tasks_path = self.config.tasks_path();
        if !tasks_path.exists() {
            std::fs::create_dir_all(tasks_path)?;
        }

        Ok(repository.find_branch(&self.config.branch, git2::BranchType::Local)?)
    }

    pub fn add_and_commit(&self, message: &str) -> Result<bool> {
        let repository = Repository::open(self.config.repository_path())?;

        if repository.statuses(None)?.is_empty() {
            return Ok(false);
        }

        let mut index = repository.index()?;

        index.add_all(["."], git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let tree = index.write_tree()?;
        let tree = repository.find_tree(tree)?;

        let author = Signature::now(&self.config.author, &self.config.email)?;

        let parent_commit = repository.head()?.peel_to_commit()?;

        let commit = repository.commit(
            Some("HEAD"),
            &author,
            &author,
            message,
            &tree,
            &[&parent_commit],
        )?;
        let commit = repository.find_commit(commit)?;

        let branch = repository.find_branch(&self.config.branch, git2::BranchType::Local)?;
        let mut branch_ref = branch.into_reference();
        branch_ref.set_target(commit.id(), "update it")?;
        let branch_ref_name = branch_ref.name().unwrap();
        repository.set_head(branch_ref_name)?;

        Ok(true)
    }

    fn resolve_conflicts(&mut self, diffs: &[TaskDiff]) -> Result<()> {
        for TaskDiff {
            path,
            adding,
            content,
        } in diffs
        {
            if !adding {
                continue;
            }

            let status = if path == Path::new("tasks/pending") {
                TaskStatus::Pending
            } else if path == Path::new("tasks/complete") {
                TaskStatus::Complete
            } else if path == Path::new("tasks/deleted") {
                TaskStatus::Deleted
            } else {
                log::warn!("skipping unknown path: {}", path.display());
                continue;
            };

            let (_, current) = self.key_store.decrypt(status, content)?;

            let Some(previous) = self.task_by_uuid(&current.uuid)? else {
                self.storage_mut()[status as usize].append(current)?;
                continue;
            };

            #[allow(clippy::match_same_arms)]
            let first = match (previous.modified, current.modified) {
                (Some(previous_modified), Some(current_modified)) => {
                    previous_modified >= current_modified
                }
                (None, Some(_)) => false,
                (Some(_), None) => true,
                (None, None) => {
                    // TODO: This should not be possible in normal circumstances.
                    //       For now choose the current.
                    true
                }
            };

            // First is already set to the working dir and tasks are loaded.
            if first {
                continue;
            }

            if previous.status == current.status {
                self.update2(&current)?;
                continue;
            }

            self.remove_task2(&previous)?;
            self.storage_mut()[status as usize].append(current)?;
        }
        Ok(())
    }

    #[allow(clippy::similar_names)]
    fn rebase(&mut self, repository: &Repository, remote: &AnnotatedCommit<'_>) -> Result<()> {
        let mut opts = RebaseOptions::new();

        let mut rebase = repository.rebase(None, Some(remote), None, Some(&mut opts))?;

        let remote_commit = repository.find_commit(remote.id())?;
        let mut patch = remote_commit.id();

        while let Some(step) = rebase.next() {
            let step = step?;

            let cid = step.id();
            let commit = repository.find_commit(cid)?;

            let base_commit = repository.find_commit(patch)?;
            let base_commit_tree = base_commit.tree()?;

            let mut checkout_options = CheckoutBuilder::new();
            checkout_options.force();
            repository.checkout_tree(base_commit_tree.as_object(), Some(&mut checkout_options))?;

            self.unload();

            let mut diffs = Vec::new();

            let diff = repository.diff_tree_to_tree(
                Some(&base_commit.tree()?),
                Some(&commit.tree()?),
                None,
            )?;
            diff.print(git2::DiffFormat::Patch, |delta, _, line| {
                if !matches!(
                    line.origin_value(),
                    git2::DiffLineType::Addition | git2::DiffLineType::Deletion
                ) {
                    return true;
                }
                let path = delta.new_file().path().unwrap();
                let content = std::str::from_utf8(line.content())
                    .unwrap()
                    .trim_end_matches('\n');
                // println!("AT:{} {} {content}", path.display(), line.origin());

                diffs.push(TaskDiff {
                    path: path.to_owned(),
                    adding: line.origin_value() == git2::DiffLineType::Addition,
                    content: content.to_owned(),
                });

                true
            })?;

            self.resolve_conflicts(&diffs)?;

            // Skip the commit if it's empty.
            if repository.statuses(None)?.is_empty() {
                log::info!(
                    "Skipping commit (empty) {cid}{}",
                    commit
                        .message()
                        .map(|x| format!(": {x}"))
                        .unwrap_or_default()
                );
                continue;
            }

            let mut index = repository.index()?;

            index.add_all(["."], git2::IndexAddOption::DEFAULT, None)?;
            index.write()?;

            let committer = Signature::now(&self.config.author, &self.config.email)?;
            patch = rebase.commit(None, &committer, None)?;
        }

        rebase.finish(None)?;

        Ok(())
    }

    pub fn pull(&mut self, repository: &Repository) -> Result<bool> {
        let mut callbacks = RemoteCallbacks::new();
        let callback_error = with_authentication(self.config.ssh_key.clone(), &mut callbacks);
        callbacks.push_update_reference(|name, status| {
            println!("{name}: {status:?}");
            Ok(())
        });

        let remote = repository.remote("origin", &self.config.origin);
        if let Err(error) = remote
            && (error.class() != ErrorClass::Config || error.code() != ErrorCode::Exists)
        {
            log::warn!("Couldn't create remote origin: {error}");
            return Err(error.into());
        }
        repository.remote_set_url("origin", &self.config.origin)?;

        let mut origin = repository.find_remote("origin")?;
        let connection = origin.connect_auth(git2::Direction::Fetch, Some(callbacks), None);

        if let Some(callback_error) = callback_error.borrow_mut().take() {
            return Err(callback_error);
        }

        let mut connection = connection?;

        let mut fetch_options = FetchOptions::new();
        let remote_result = connection.remote().fetch(
            &[self.config.branch.as_ref()],
            Some(&mut fetch_options),
            Some(&format!("fetch {} branch", self.config.branch)),
        );

        if let Some(callback_error) = callback_error.borrow_mut().take() {
            return Err(callback_error);
        }

        remote_result?;

        let mut local = repository.find_branch(&self.config.branch, git2::BranchType::Local)?;
        let remote = match local.upstream() {
            Ok(remote) => remote.into_reference(),
            Err(error)
                if error.class() == ErrorClass::Config && error.code() == ErrorCode::NotFound =>
            {
                local.set_upstream(Some(&format!("origin/{}", self.config.branch)))?;
                local.upstream()?.into_reference()
            }
            Err(error) => return Err(error.into()),
        };

        let (ahead, behind) = repository.graph_ahead_behind(
            local.into_reference().peel_to_commit()?.id(),
            remote.peel_to_commit()?.id(),
        )?;

        if behind == 0 {
            return Ok(false);
        }

        let remote = repository.reference_to_annotated_commit(&remote)?;

        if ahead != 0 {
            self.rebase(repository, &remote)?;
            return Ok(true);
        }

        let fetch_head = repository.find_reference("FETCH_HEAD")?;
        let fetch_commit = repository.reference_to_annotated_commit(&fetch_head)?;

        let remote_branch = &self.config.branch;
        do_merge(repository, remote_branch, &fetch_commit)?;

        Ok(true)
    }

    pub fn push(&self, repository: &Repository, force: bool) -> Result<()> {
        let mut callbacks = RemoteCallbacks::new();
        let callback_error = with_authentication(self.config.ssh_key.clone(), &mut callbacks);
        callbacks.push_update_reference(|name, status| {
            println!("{name}: {status:?}");
            Ok(())
        });

        let remote = repository.remote("origin", &self.config.origin);
        if let Err(error) = remote
            && (error.class() != ErrorClass::Config || error.code() != ErrorCode::Exists)
        {
            log::warn!("Couldn't create remote origin: {error}");
            return Err(error.into());
        }
        repository.remote_set_url("origin", &self.config.origin)?;

        let mut origin = repository.find_remote("origin")?;
        let connection = origin.connect_auth(git2::Direction::Push, Some(callbacks), None);

        if let Some(callback_error) = callback_error.borrow_mut().take() {
            return Err(callback_error);
        }

        let mut connection = connection?;

        let local_branch = match repository
            .find_branch(&self.config.branch, git2::BranchType::Local)
        {
            Ok(value) => value,
            Err(err)
                if err.class() == ErrorClass::Reference && err.code() == ErrorCode::NotFound =>
            {
                self.init_repotitory(repository)?
            }
            Err(err) => return Err(err.into()),
        };

        let branch_ref = local_branch.into_reference();
        let mut branch_ref_name = branch_ref.name().unwrap().to_owned();

        // https://github.com/libgit2/libgit2/issues/4286
        // The '+' means force push.
        if force {
            log::info!("Force pushing: {branch_ref_name}");
            branch_ref_name = format!("+{branch_ref_name}");
        }

        let remote_result = connection.remote().push(&[&branch_ref_name], None);

        if let Some(callback_error) = callback_error.borrow_mut().take() {
            return Err(callback_error);
        }

        remote_result?;

        Ok(())
    }

    pub fn delete_all(&mut self) -> Result<()> {
        for storage in self.storage_mut() {
            storage.clear()?;
        }
        // delete repository root directory.
        std::fs::remove_dir_all(&self.config.root_path)?;
        Ok(())
    }
}

impl GitBackend {
    fn unload(&mut self) {
        for storage in self.storage_mut() {
            storage.unload();
        }
    }

    fn add(&mut self, task: Task) -> Result<()> {
        let message = format!("$ADD {}", task.uuid.to_base64());
        self.pending.append(task)?;
        self.add_and_commit(&message)?;
        Ok(())
    }

    pub fn remove_by_uuid(&mut self, uuid: &Uuid) -> Result<Option<Task>> {
        for storage in self.storage_mut() {
            if let Some(task) = storage.remove(uuid)? {
                return Ok(Some(task));
            }
        }
        Ok(None)
    }

    pub fn remove_by_task(&mut self, task: &Task) -> Result<bool> {
        let found_task = self.remove_task2(task)?;

        let Some(found_task) = found_task else {
            return Ok(false);
        };

        let message = format!("$PURGE {}", found_task.uuid.to_base64());

        self.add_and_commit(&message)?;
        Ok(false)
    }

    fn task_by_uuid(&mut self, uuid: &Uuid) -> Result<Option<Task>> {
        for storage in self.storage_mut() {
            let task = storage.get_by_id(uuid)?;

            if let Some(task) = task {
                return Ok(Some(task.clone()));
            }
        }
        Ok(None)
    }

    // fn tasks_with_filter(&mut self, filter: &Filter) -> Result<Vec<Task>> {
    //     let mut tasks = Vec::new();
    //     for storage in self.storage_mut() {
    //         storage.filter(filter, &mut tasks)?;
    //     }
    //     tasks.sort_unstable_by(|a, b| {
    //         b.urgency()
    //             .partial_cmp(&a.urgency())
    //             .expect("should never be NaN")
    //     });
    //     Ok(tasks)
    // }

    fn update(&mut self, task: &Task) -> Result<bool> {
        let mut updated = false;
        if let Some(found_task) = self.task_by_uuid(&task.uuid)? {
            updated |= self.change_category(&found_task, task.status)?;
        }

        updated |= self.update2(task)?;
        if updated {
            self.add_and_commit(&format!("$UPDATE {}", task.uuid.to_base64()))?;
        }
        Ok(updated)
    }
}

impl Backend for GitBackend {
    fn handler() -> Box<dyn BackendHandler>
    where
        Self: Sized,
    {
        Box::new(Handler)
    }

    fn sync(&mut self, db: &mut Database) -> Result<(), stride_backend::Error> {
        let tasks = db.all_tasks()?;

        if self.config.repository_path().join(".git").exists() {
            for task in tasks {
                if self.task_by_uuid(&task.uuid)?.is_some() {
                    self.update(&task)?;
                } else {
                    self.add(task)?;
                }
            }
            let repository =
                Repository::open(self.config.repository_path()).map_err(Error::LibGit2)?;
            log::info!("Pulling tasks...");
            if self.pull(&repository)? {
                log::info!("Pulled tasks");
                self.unload();
            }
            log::info!("Pushing tasks...");
            self.push(&repository, false)?;

            log::info!("Task sync finished!");
            Ok(())
        } else {
            self.clone_repository()?;
            for task in tasks {
                if self.task_by_uuid(&task.uuid)?.is_some() {
                    self.update(&task)?;
                } else {
                    self.add(task)?;
                }
            }
            self.sync(db)
        }
    }
}
impl GitBackend {
    pub fn clear(&mut self) -> Result<()> {
        for storage in self.storage_mut() {
            storage.clear()?;
        }
        // delete repository root directory.
        std::fs::remove_dir_all(self.config.repository_path())?;
        Ok(())
    }

    // fn export(&mut self) -> Result<String> {
    //     #[derive(serde::Serialize)]
    //     struct ExportTask<'a> {
    //         #[serde(skip_serializing_if = "<[_]>::is_empty")]
    //         pending: &'a [DecryptedTask],
    //         #[serde(skip_serializing_if = "<[_]>::is_empty")]
    //         complete: &'a [DecryptedTask],
    //         #[serde(skip_serializing_if = "<[_]>::is_empty")]
    //         deleted: &'a [DecryptedTask],
    //         #[serde(skip_serializing_if = "<[_]>::is_empty")]
    //         waiting: &'a [DecryptedTask],
    //         #[serde(skip_serializing_if = "<[_]>::is_empty")]
    //         recurring: &'a [DecryptedTask],
    //     }

    //     self.init_repository_if_needed()?;

    //     for storage in self.storage_mut() {
    //         storage.load()?;
    //     }

    //     let record = ExportTask {
    //         pending: &self.pending.tasks,
    //         complete: &self.complete.tasks,
    //         deleted: &self.deleted.tasks,
    //         waiting: &self.waiting.tasks,
    //         recurring: &self.recurring.tasks,
    //     };

    //     Ok(serde_json::to_string(&record).map_err(ExportError::Serialize)?)
    // }

    // fn import(&mut self, content: &str) -> Result<()> {
    //     #[derive(serde::Deserialize)]
    //     struct ImportRecord {
    //         #[serde(default)]
    //         pending: Vec<DecryptedTask>,
    //         #[serde(default)]
    //         complete: Vec<DecryptedTask>,
    //         #[serde(default)]
    //         deleted: Vec<DecryptedTask>,
    //         #[serde(default)]
    //         waiting: Vec<DecryptedTask>,
    //         #[serde(default)]
    //         recurring: Vec<DecryptedTask>,
    //     }

    //     self.init_repository_if_needed()?;

    //     let record: ImportRecord =
    //         serde_json::from_str(content).map_err(ImportError::Deserialize)?;

    //     self.pending.tasks = record.pending;
    //     self.pending.loaded = true;
    //     self.complete.tasks = record.complete;
    //     self.complete.loaded = true;
    //     self.deleted.tasks = record.deleted;
    //     self.deleted.loaded = true;
    //     self.waiting.tasks = record.waiting;
    //     self.waiting.loaded = true;
    //     self.recurring.tasks = record.recurring;
    //     self.recurring.loaded = true;

    //     for storage in self.storage_mut() {
    //         storage.save()?;
    //     }

    //     Ok(())
    // }

    pub fn query(&mut self, query: &TaskQuery) -> Result<Vec<Task>> {
        let mut result = Vec::new();
        for storage in self.storage_mut() {
            storage.query(query, &mut result)?;
        }
        Ok(result)
    }
}

fn with_authentication(
    ssh_key: SshKey,
    callbacks: &mut RemoteCallbacks<'_>,
) -> Rc<RefCell<Option<Error>>> {
    let mut tried_ssh = false;

    let error = Rc::<RefCell<Option<Error>>>::default();

    // See: https://github.com/rust-lang/git2-rs/issues/347
    callbacks.credentials(move |_url, username_from_url, _allowed_types| {
        if tried_ssh {
            log::error!("Failed to authenticate with credentials");
            return Err(git2::Error::new(
                ErrorCode::Auth,
                ErrorClass::Ssh,
                "Failed to authenticate with credentials",
            ));
        }
        let Some(_username) = username_from_url else {
            return Err(git2::Error::new(
                ErrorCode::Auth,
                ErrorClass::Ssh,
                "No username provide in the url",
            ));
        };

        tried_ssh = true;
        Cred::ssh_key(
            username_from_url.unwrap(),
            Some(&ssh_key.public_path),
            &ssh_key.private_path,
            None,
        )
    });

    let known_hosts = KnownHosts::load().unwrap();

    let certificate_error = error.clone();
    callbacks.certificate_check(move |cert, hostname| {
        let Some(cert_host_key) = cert.as_hostkey() else {
            return Ok(CertificateCheckStatus::CertificatePassthrough);
        };
        let Some(host_key_type) = cert_host_key.hostkey_type() else {
            *certificate_error.borrow_mut() = Some(Error::MissingHostKey {
                hostname: hostname.into(),
            });
            return Err(git2::Error::new(
                ErrorCode::Certificate,
                ErrorClass::Callback,
                "remote host key is not available",
            ));
        };
        let host_key = cert_host_key.hostkey().unwrap();
        let host_key = base64::engine::general_purpose::STANDARD.encode(host_key);

        let Ok(host_key_type) = HostKeyType::try_from(host_key_type) else {
            *certificate_error.borrow_mut() = Some(Error::UnknownKeyType);
            return Err(git2::Error::new(
                ErrorCode::Certificate,
                ErrorClass::Callback,
                "unknown remote key type",
            ));
        };

        let Some(host) = known_hosts.host(hostname, host_key_type) else {
            *certificate_error.borrow_mut() = Some(Error::UnknownHost {
                host: Host::new(hostname.to_owned(), host_key_type, host_key),
            });
            return Err(git2::Error::new(
                ErrorCode::Certificate,
                ErrorClass::Callback,
                "unknown host",
            ));
        };

        if host.key != host_key {
            *certificate_error.borrow_mut() = Some(Error::MismatchRemoteKey {
                expected: host.key.clone().into_boxed_str(),
                actual: host_key.into(),
            });
            return Err(git2::Error::new(
                ErrorCode::Certificate,
                ErrorClass::Callback,
                "mismatched host key",
            ));
        }

        Ok(CertificateCheckStatus::CertificateOk)
    });

    error
}

fn fast_forward(
    repo: &Repository,
    lb: &mut git2::Reference<'_>,
    rc: &AnnotatedCommit<'_>,
) -> Result<(), git2::Error> {
    let name = match lb.name() {
        Some(s) => s.to_string(),
        None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
    };
    let msg = format!("Fast-Forward: Setting {} to id: {}", name, rc.id());
    log::info!("{msg}");

    lb.set_target(rc.id(), &msg)?;
    repo.set_head(&name)?;
    repo.checkout_head(Some(
        CheckoutBuilder::default()
            // For some reason the force is required to make the working directory actually get updated
            // I suspect we should be adding some logic to handle dirty working directory states
            // but this is just an example so maybe not.
            .force(),
    ))?;
    Ok(())
}

fn do_merge<'a>(
    repo: &'a Repository,
    remote_branch: &str,
    fetch_commit: &AnnotatedCommit<'a>,
) -> Result<bool, git2::Error> {
    // 1. do a merge analysis
    let (analysis, _) = repo.merge_analysis(&[fetch_commit])?;

    // 2. Do the appropriate merge
    if !analysis.is_fast_forward() {
        return Ok(false);
    }

    if analysis.is_up_to_date() {
        return Ok(false);
    }

    log::trace!("Doing a fast forward");
    // do a fast forward
    let refname = format!("refs/heads/{remote_branch}");
    match repo.find_reference(&refname) {
        Ok(mut r) => {
            fast_forward(repo, &mut r, fetch_commit)?;
        }
        Err(_e) => {
            // The branch doesn't exist so just set the reference to the
            // commit directly. Usually this is because you are pulling
            // into an empty repository.
            repo.reference(
                &refname,
                fetch_commit.id(),
                true,
                &format!("Setting {} to {}", remote_branch, fetch_commit.id()),
            )?;
            repo.set_head(&refname)?;
            repo.checkout_head(Some(
                CheckoutBuilder::default()
                    .allow_conflicts(true)
                    .conflict_style_merge(true)
                    .force(),
            ))?;
        }
    }

    Ok(true)
}

// https://github.com/fzyzcjy/flutter_rust_bridge/issues/1937
#[allow(missing_debug_implementations)]
pub struct CommitItem {
    pub oid: Oid,
    pub parent: Option<Oid>,
    pub message: String,
    pub author: String,
    pub email: String,
}

#[allow(missing_debug_implementations)]
pub struct LogIter<'repo> {
    repository: &'repo Repository,
    revwalk: git2::Revwalk<'repo>,
}

impl Iterator for LogIter<'_> {
    type Item = Result<CommitItem>;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self
            .revwalk
            .next()?
            .and_then(|oid| self.repository.find_commit(oid))
            .map_err(Error::from)
            .map(|commit| CommitItem {
                oid: commit.id(),
                parent: commit.parent_ids().next(),
                message: commit.message().unwrap_or("<non-utf8>").into(),
                author: commit.author().name().unwrap_or("<non-utf8>").into(),
                email: commit.author().email().unwrap_or("<non-utf8>").into(),
            });

        Some(item)
    }
}

impl GitBackend {
    pub fn log(&mut self, oid: Option<Oid>, n: Option<u32>) -> Result<Option<Vec<CommitItem>>> {
        let repository = match Repository::open(self.config.repository_path()) {
            Ok(repository) => repository,
            Err(error) if error.code() == ErrorCode::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        let mut revwalk = repository.revwalk()?;

        if let Some(oid) = oid {
            revwalk.push(oid)?;
        } else {
            revwalk.push_head()?;
        }

        let mut commits = Vec::new();
        for commit in (LogIter {
            repository: &repository,
            revwalk,
        })
        .take(n.unwrap_or(u32::MAX) as usize)
        {
            commits.push(commit?);
        }

        Ok(Some(commits))
    }
}
