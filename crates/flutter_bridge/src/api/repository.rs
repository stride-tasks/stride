use std::{
    cell::RefCell,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

use anyhow::{Context, Result};
use base64::Engine;
use chrono::Utc;
use flutter_rust_bridge::frb;
use uuid::Uuid;

use crate::{
    api::{
        error::{ExportError, ImportError},
        settings::Settings,
    },
    git::known_hosts::{Host, HostKeyType, KnownHosts},
    task::{Task, TaskPriority, TaskStatus},
    ErrorKind, RustError, ToBase64,
};

use git2::{
    build::CheckoutBuilder, AnnotatedCommit, CertificateCheckStatus, Cred, ErrorClass, ErrorCode,
    FetchOptions, RebaseOptions, RemoteCallbacks, Repository, Signature,
};

use super::{
    filter::Filter,
    logging::Logger,
    settings::{ssh_key, SshKey},
};

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

impl Task {
    #[frb(sync)]
    pub fn new(title: String) -> Self {
        Task {
            title,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_uuid(uuid: Uuid, title: String) -> Self {
        Task {
            uuid,
            title,
            ..Default::default()
        }
    }

    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    #[frb(sync)]
    pub fn urgency(&self) -> f32 {
        const THREE_DAYS: i64 = 3 * 24 * 60 * 60;

        let mut urgency = 0.0;
        urgency += f32::from(self.active) * 15.0;
        if let Some(due) = self.due {
            let today = Utc::now();
            let delta = due - today;

            urgency += 1.0;

            let seconds = delta.num_seconds();
            if seconds < 0 {
                urgency += 11.0;
            } else if seconds <= THREE_DAYS {
                urgency += (seconds as f32 / THREE_DAYS as f32) * 11.0;
            }
        }
        if let Some(priority) = self.priority {
            match priority {
                TaskPriority::H => urgency += 6.0,
                TaskPriority::M => urgency += 3.0,
                TaskPriority::L => urgency += -3.0,
            }
        }
        urgency
    }
}

pub(crate) struct TaskDiff {
    path: PathBuf,
    adding: bool,
    content: String,
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

    fn load(&mut self) -> Result<(), RustError> {
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
            let mut task = Task::from_data(&line).ok_or(ErrorKind::CorruptTask)?;
            task.status = self.kind;

            tasks.push(task);
        }

        self.tasks = tasks;
        self.loaded = true;
        Ok(())
    }

    fn append(&mut self, mut task: Task) -> Result<(), RustError> {
        task.status = self.kind;

        let mut file = File::options().append(true).create(true).open(&self.path)?;

        let mut content = task.to_data();
        content.push('\n');
        file.write_all(content.as_bytes())?;
        self.tasks.push(task);
        Ok(())
    }

    fn save(&mut self) -> Result<(), RustError> {
        let mut content = String::new();
        for task in &self.tasks {
            content += &task.to_data();
            content.push('\n');
        }

        std::fs::write(&self.path, content)?;
        Ok(())
    }

    fn get_by_id(&mut self, uuid: &Uuid) -> Result<Option<&Task>, RustError> {
        self.load()?;
        Ok(self.tasks.iter().find(|task| &task.uuid == uuid))
    }

    #[allow(unused)]
    fn get_index(&mut self, uuid: &Uuid) -> Result<Option<usize>, RustError> {
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
            .filter(|task| task.title.contains(&filter.search))
        {
            result.push(task.clone());
        }

        Ok(())
    }

    fn update(&mut self, task: &Task) -> Result<bool, RustError> {
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
        current.modified = Some(Utc::now());

        self.save()?;
        Ok(true)
    }

    fn remove(&mut self, uuid: &Uuid) -> Result<Option<Task>> {
        let index = self.get_index(uuid)?;
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
        self.loaded = false;
        self.tasks.clear();
    }
}

#[frb(opaque)]
pub struct TaskStorage {
    pub(crate) repository_path: PathBuf,
    tasks_path: PathBuf,

    pending: Storage,
    complete: Storage,
    deleted: Storage,
    waiting: Storage,
    recurring: Storage,
}

impl TaskStorage {
    const PENDING_DATA_FILENAME: &'static str = "pending";
    const COMPLETE_DATA_FILENAME: &'static str = "complete";
    const DELETED_DATA_FILENAME: &'static str = "deleted";
    const WAITING_DATA_FILENAME: &'static str = "waiting";
    const RECURRING_DATA_FILENAME: &'static str = "recurring";

    #[frb(sync)]
    pub fn new(path: &str) -> Self {
        let repository_path = Path::new(path);
        let tasks_path = repository_path.join("tasks");
        Self {
            repository_path: repository_path.to_path_buf(),
            pending: Storage::new(
                tasks_path.join(Self::PENDING_DATA_FILENAME),
                TaskStatus::Pending,
            ),
            complete: Storage::new(
                tasks_path.join(Self::COMPLETE_DATA_FILENAME),
                TaskStatus::Complete,
            ),
            deleted: Storage::new(
                tasks_path.join(Self::DELETED_DATA_FILENAME),
                TaskStatus::Deleted,
            ),
            waiting: Storage::new(
                tasks_path.join(Self::WAITING_DATA_FILENAME),
                TaskStatus::Waiting,
            ),
            recurring: Storage::new(
                tasks_path.join(Self::RECURRING_DATA_FILENAME),
                TaskStatus::Recurring,
            ),
            tasks_path,
        }
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

    pub fn unload(&mut self) {
        for storage in self.storage_mut() {
            storage.unload();
        }
    }

    pub fn add(&mut self, task: Task) -> Result<()> {
        if !self.repository_path.exists() {
            self.init_repotitory()?;
        }

        let message = format!("$ADD {}", task.uuid.to_base64());
        self.pending.append(task)?;
        self.add_and_commit(&message)?;
        Ok(())
    }

    pub fn task_by_uuid(&mut self, uuid: &Uuid) -> Result<Option<Task>, RustError> {
        for storage in self.storage_mut() {
            let task = storage.get_by_id(uuid)?;

            if let Some(task) = task {
                return Ok(Some(task.clone()));
            }
        }
        Ok(None)
    }

    pub fn tasks_with_filter(&mut self, filter: &Filter) -> Result<Vec<Task>> {
        let mut tasks = Vec::new();
        for storage in self.storage_mut() {
            storage.filter(filter, &mut tasks)?;
        }

        tasks.sort_unstable_by(|a, b| {
            b.urgency()
                .partial_cmp(&a.urgency())
                .expect("should never be NaN")
        });

        Ok(tasks)
    }

    #[allow(unused)]
    pub(crate) fn remove(&mut self, uuid: &Uuid) -> Result<Option<Task>> {
        for storage in self.storage_mut() {
            if let Some(task) = storage.remove(uuid)? {
                return Ok(Some(task));
            }
        }
        Ok(None)
    }

    pub(crate) fn update2(&mut self, task: &Task) -> Result<bool, RustError> {
        let mut updated = false;
        for storage in self.storage_mut() {
            if storage.update(task)? {
                updated = true;
                break;
            }
        }
        Ok(updated)
    }

    pub fn update(&mut self, task: &Task) -> Result<bool> {
        let updated = self.update2(task)?;
        if updated {
            self.add_and_commit(&format!("$UPDATE {}", task.uuid.to_base64()))?;
        }
        Ok(updated)
    }

    pub fn change_category(&mut self, task: &Task, status: TaskStatus) -> Result<bool> {
        if task.status == status {
            return Ok(true);
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

        let mut found_task =
            found_task.with_context(|| format!("No task found with uuid: {}", task.uuid))?;

        found_task.active = false;
        found_task.status = status;
        found_task.modified = Some(Utc::now());

        let transition = match status {
            TaskStatus::Pending => "PEND",
            TaskStatus::Waiting => "WAIT",
            TaskStatus::Recurring => "RECUR",
            TaskStatus::Deleted => "DELETE",
            TaskStatus::Complete => "DONE",
        };

        let message = format!("${transition} {}", found_task.uuid.to_base64());
        self.storage_mut()[status as usize].append(found_task)?;
        self.add_and_commit(&message)?;
        Ok(false)
    }

    pub(crate) fn remove_task2(&mut self, task: &Task) -> Result<Option<Task>, RustError> {
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
        Ok(found_task)
    }

    pub fn remove_task(&mut self, task: &Task) -> Result<bool> {
        let found_task = self.remove_task2(task)?;

        let found_task =
            found_task.with_context(|| format!("No task found with uuid: {}", task.uuid))?;

        let message = format!("$PURGE {}", found_task.uuid.to_base64());

        self.add_and_commit(&message)?;
        Ok(false)
    }

    pub fn tasks(&mut self) -> Result<Vec<Task>> {
        self.tasks_with_filter(&Filter {
            name: "default".to_owned(),
            status: HashSet::from_iter([TaskStatus::Pending]),
            uuid: Uuid::now_v7(),
            search: String::new(),
        })
    }

    pub fn sync(&mut self) -> Result<(), RustError> {
        if self.repository_path.exists() {
            // TODO: Make sure that nothing is left behind!

            if self.pull()? {
                log::info!("Pulled tasks");
                self.unload();
            }

            self.push(false)?;

            log::info!("Task sync finished!");
            Ok(())
        } else {
            self.clone_repository()
        }
    }

    pub fn clear(&mut self) -> Result<()> {
        for storage in self.storage_mut() {
            storage.clear()?;
        }
        std::fs::remove_dir_all(&self.repository_path)?;
        Ok(())
    }

    pub fn clone_repository(&mut self) -> Result<(), RustError> {
        let settings = Settings::get();

        let ssh_key = Self::ssh_key(&settings)?;

        let mut callbacks = RemoteCallbacks::new();
        let callback_error =
            with_authentication(ssh_key, settings.known_hosts.clone(), &mut callbacks);

        let mut fo = FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fo);
        builder.branch(&settings.repository.branch);

        let connection = builder.clone(&settings.repository.origin, &self.repository_path);

        if let Some(callback_error) = callback_error.borrow_mut().take() {
            return Err(callback_error);
        }

        if let Err(error) = &connection {
            return match error.class() {
                ErrorClass::Ssh => Err(ErrorKind::Authentication {
                    message: error.message().into(),
                }
                .into()),
                ErrorClass::Net => Err(ErrorKind::Network {
                    message: error.message().into(),
                }
                .into()),
                _ => Err(ErrorKind::Other {
                    message: error.message().into(),
                }
                .into()),
            };
        }

        log::info!(
            "Repository {} cloned successfully!",
            settings.repository.origin
        );

        self.unload();

        if !self.tasks_path.exists() {
            std::fs::create_dir(&self.tasks_path)
                .expect("creating the tasks directory should not fail");
        }

        Ok(())
    }

    pub fn force_hard_reset(&mut self, commit: Oid) -> Result<(), RustError> {
        let settings = Settings::get();

        let repository = Repository::open(&self.repository_path)?;
        let commit = repository.find_commit(commit)?;

        let branch =
            repository.find_branch(&settings.repository.branch, git2::BranchType::Local)?;

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

    pub fn checkout(&mut self) -> Result<()> {
        let settings = Settings::get();

        self.sync()?;
        self.unload();

        let repository = Repository::open(&self.repository_path)?;
        let branch =
            repository.find_branch(&settings.repository.branch, git2::BranchType::Local)?;
        let reference = branch.into_reference();
        let tree = reference.peel_to_tree()?;
        repository.checkout_tree(tree.as_object(), None)?;

        let name = reference
            .name()
            .context("invalid UTF-8 reference name of branch")?;
        repository.set_head(name)?;

        if !self.tasks_path.exists() {
            std::fs::create_dir(&self.tasks_path)
                .expect("creating the tasks directory should not fail");
        }

        Ok(())
    }

    pub fn init_repotitory(&self) -> Result<(), RustError> {
        let settings = Settings::get();

        let repository = Repository::init(&self.repository_path)?;

        let mut index = repository.index()?;

        let tree = index.write_tree()?;
        let tree = repository.find_tree(tree)?;

        let author = Signature::now(&settings.repository.author, &settings.repository.email)?;

        let commit = repository.commit(None, &author, &author, "Initial Commit", &tree, &[])?;
        let commit = repository.find_commit(commit)?;

        let branch = repository.branch(&settings.repository.branch, &commit, true)?;
        let mut branch_ref = branch.into_reference();
        branch_ref.set_target(commit.id(), "update it")?;
        let branch_ref_name = branch_ref.name().unwrap();
        repository.set_head(branch_ref_name)?;

        if !self.tasks_path.exists() {
            std::fs::create_dir_all(&self.tasks_path)?;
        }

        Result::Ok(())
    }

    pub fn add_and_commit(&self, message: &str) -> Result<bool, RustError> {
        let settings = Settings::get();

        let repository = Repository::open(&self.repository_path)?;

        if repository.statuses(None)?.is_empty() {
            return Ok(false);
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
            message,
            &tree,
            &[&parent_commit],
        )?;
        let commit = repository.find_commit(commit)?;

        let branch =
            repository.find_branch(&settings.repository.branch, git2::BranchType::Local)?;
        let mut branch_ref = branch.into_reference();
        branch_ref.set_target(commit.id(), "update it")?;
        let branch_ref_name = branch_ref.name().unwrap();
        repository.set_head(branch_ref_name)?;

        Ok(true)
    }

    fn resolve_conflicts(&mut self, diffs: &[TaskDiff]) -> Result<(), RustError> {
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

            let mut current = Task::from_data(content).ok_or(ErrorKind::CorruptTask)?;
            current.status = status;

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

    fn rebase(
        &mut self,
        settings: &Settings,
        repository: &Repository,
        remote: &AnnotatedCommit<'_>,
    ) -> Result<(), RustError> {
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

            let committer =
                Signature::now(&settings.repository.author, &settings.repository.email)?;
            patch = rebase.commit(None, &committer, None)?;
        }

        rebase.finish(None)?;

        Ok(())
    }

    fn ssh_key(settings: &Settings) -> Result<SshKey, RustError> {
        let Some(uuid) = settings.repository.ssh_key_uuid else {
            return Err(ErrorKind::NoSshKeysProvided.into());
        };
        let Some((public_path, private_path)) = ssh_key(&uuid) else {
            // TODO: Report a different error, missing key.
            return Err(ErrorKind::NoSshKeysProvided.into());
        };

        let public_key = std::fs::read_to_string(&public_path)?;

        Ok(SshKey {
            uuid,
            public_key,
            public_path,
            private_path,
        })
    }

    #[frb(ignore)]
    pub fn pull(&mut self) -> Result<bool, RustError> {
        let settings = Settings::get();

        let repository = Repository::open(&self.repository_path)?;

        let ssh_key = Self::ssh_key(&settings)?;
        let mut callbacks = RemoteCallbacks::new();
        let callback_error =
            with_authentication(ssh_key, settings.known_hosts.clone(), &mut callbacks);
        callbacks.push_update_reference(|name, status| {
            println!("{name}: {status:?}");
            Result::Ok(())
        });

        let remote = repository.remote("origin", &settings.repository.origin);
        if let Err(error) = remote {
            if error.class() != ErrorClass::Config || error.code() != ErrorCode::Exists {
                log::warn!("Couldn't create remote origin: {error}");
                return Err(error.into());
            }
        }
        repository.remote_set_url("origin", &settings.repository.origin)?;

        let mut origin = repository.find_remote("origin")?;
        let connection = origin.connect_auth(git2::Direction::Fetch, Some(callbacks), None);

        if let Some(callback_error) = callback_error.borrow_mut().take() {
            return Err(callback_error);
        }

        if let Err(error) = &connection {
            return match error.class() {
                ErrorClass::Ssh => Err(ErrorKind::Authentication {
                    message: error.message().into(),
                }
                .into()),
                ErrorClass::Net => Err(ErrorKind::Network {
                    message: error.message().into(),
                }
                .into()),
                _ => Err(ErrorKind::Other {
                    message: error.message().into(),
                }
                .into()),
            };
        }

        let branch =
            repository.find_branch(&settings.repository.branch, git2::BranchType::Local)?;
        let branch_ref = branch.into_reference();
        let branch_ref_name = branch_ref.name().unwrap();
        let mut fetch_options = FetchOptions::new();
        fetch_options.prune(git2::FetchPrune::On);
        fetch_options.download_tags(git2::AutotagOption::All);
        let connection =
            connection?
                .remote()
                .fetch(&[branch_ref_name], Some(&mut fetch_options), None);

        if let Some(callback_error) = callback_error.borrow_mut().take() {
            return Err(callback_error);
        }

        if let Err(error) = &connection {
            return match error.class() {
                ErrorClass::Ssh => Err(ErrorKind::Authentication {
                    message: error.message().into(),
                }
                .into()),
                ErrorClass::Net => Err(ErrorKind::Network {
                    message: error.message().into(),
                }
                .into()),
                _ => Err(ErrorKind::Other {
                    message: error.message().into(),
                }
                .into()),
            };
        }

        let local = repository.find_branch(&settings.repository.branch, git2::BranchType::Local)?;
        let remote = local.upstream()?;
        let remote = remote.into_reference();
        let (ahead, behind) = repository.graph_ahead_behind(
            local.into_reference().peel_to_commit()?.id(),
            remote.peel_to_commit()?.id(),
        )?;

        if behind == 0 {
            return Ok(false);
        }

        let remote = repository.reference_to_annotated_commit(&remote)?;

        if ahead != 0 {
            self.rebase(&settings, &repository, &remote)?;
            return Ok(true);
        }

        let fetch_head = repository.find_reference("FETCH_HEAD")?;
        let fetch_commit = repository.reference_to_annotated_commit(&fetch_head)?;

        let remote_branch = &settings.repository.branch;
        do_merge(&repository, remote_branch, &fetch_commit)?;

        Result::Ok(true)
    }

    pub fn push(&self, force: bool) -> Result<(), RustError> {
        let settings = Settings::get();

        let repository = Repository::open(&self.repository_path)?;

        let ssh_key = Self::ssh_key(&settings)?;
        let mut callbacks = RemoteCallbacks::new();
        let callback_error =
            with_authentication(ssh_key, settings.known_hosts.clone(), &mut callbacks);
        callbacks.push_update_reference(|name, status| {
            println!("{name}: {status:?}");
            Result::Ok(())
        });

        let remote = repository.remote("origin", &settings.repository.origin);
        if let Err(error) = remote {
            if error.class() != ErrorClass::Config || error.code() != ErrorCode::Exists {
                log::warn!("Couldn't create remote origin: {error}");
                return Err(error.into());
            }
        }
        repository.remote_set_url("origin", &settings.repository.origin)?;

        let mut origin = repository.find_remote("origin")?;
        let connection = origin.connect_auth(git2::Direction::Push, Some(callbacks), None);

        if let Some(callback_error) = callback_error.borrow_mut().take() {
            return Err(callback_error);
        }

        let mut connection = match connection {
            Ok(connection) => connection,
            Err(error) => {
                return match error.class() {
                    ErrorClass::Ssh => Err(ErrorKind::Authentication {
                        message: error.message().into(),
                    }
                    .into()),
                    ErrorClass::Net => Err(ErrorKind::Network {
                        message: error.message().into(),
                    }
                    .into()),
                    _ => Err(error.into()),
                }
            }
        };

        let local_branch =
            repository.find_branch(&settings.repository.branch, git2::BranchType::Local)?;
        let branch_ref = local_branch.into_reference();
        let mut branch_ref_name = branch_ref.name().unwrap().to_owned();

        // https://github.com/libgit2/libgit2/issues/4286
        // The '+' means force push.
        if force {
            Logger::info(&format!("Force pushing: {branch_ref_name}"));
            branch_ref_name = format!("+{branch_ref_name}");
        }

        let connection = connection.remote().push(&[&branch_ref_name], None);

        if let Some(callback_error) = callback_error.borrow_mut().take() {
            return Err(callback_error);
        }

        if let Err(error) = connection {
            return match error.class() {
                ErrorClass::Ssh => Err(ErrorKind::Authentication {
                    message: error.message().into(),
                }
                .into()),
                ErrorClass::Net => Err(ErrorKind::Network {
                    message: error.message().into(),
                }
                .into()),
                _ => Err(error.into()),
            };
        }

        Result::Ok(())
    }

    pub fn export(&mut self) -> Result<String, RustError> {
        #[derive(serde::Serialize)]
        struct ExportTask<'a> {
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            pending: &'a [Task],
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            complete: &'a [Task],
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            deleted: &'a [Task],
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            waiting: &'a [Task],
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            recurring: &'a [Task],
        }

        for storage in self.storage_mut() {
            storage.load()?;
        }

        let record = ExportTask {
            pending: &self.pending.tasks,
            complete: &self.complete.tasks,
            deleted: &self.deleted.tasks,
            waiting: &self.waiting.tasks,
            recurring: &self.recurring.tasks,
        };

        Ok(serde_json::to_string(&record).map_err(ExportError::Serialize)?)
    }

    pub fn import(&mut self, content: &str) -> Result<(), RustError> {
        #[derive(serde::Deserialize)]
        struct ImportRecord {
            #[serde(default)]
            pending: Vec<Task>,
            #[serde(default)]
            complete: Vec<Task>,
            #[serde(default)]
            deleted: Vec<Task>,
            #[serde(default)]
            waiting: Vec<Task>,
            #[serde(default)]
            recurring: Vec<Task>,
        }

        let record: ImportRecord =
            serde_json::from_str(content).map_err(ImportError::Deserialize)?;

        self.pending.tasks = record.pending;
        self.pending.loaded = true;
        self.complete.tasks = record.complete;
        self.complete.loaded = true;
        self.deleted.tasks = record.deleted;
        self.deleted.loaded = true;
        self.waiting.tasks = record.waiting;
        self.waiting.loaded = true;
        self.recurring.tasks = record.recurring;
        self.recurring.loaded = true;

        for storage in self.storage_mut() {
            storage.save()?;
        }

        Ok(())
    }
}

fn with_authentication(
    ssh_key: SshKey,
    known_hosts: KnownHosts,
    callbacks: &mut RemoteCallbacks<'_>,
) -> Rc<RefCell<Option<RustError>>> {
    let mut tried_ssh = false;

    let error = Rc::<RefCell<Option<RustError>>>::default();

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

    let certificate_error = error.clone();
    callbacks.certificate_check(move |cert, hostname| {
        let Some(cert_host_key) = cert.as_hostkey() else {
            return Result::Ok(CertificateCheckStatus::CertificatePassthrough);
        };
        let Some(host_key_type) = cert_host_key.hostkey_type() else {
            *certificate_error.borrow_mut() = Some(
                ErrorKind::MissingHostKey {
                    hostname: hostname.into(),
                }
                .into(),
            );
            return Err(git2::Error::new(
                ErrorCode::Certificate,
                ErrorClass::Callback,
                "remote host key is not available",
            ));
        };
        let host_key = cert_host_key.hostkey().unwrap();
        let host_key = base64::engine::general_purpose::STANDARD.encode(host_key);

        let Result::Ok(host_key_type) = HostKeyType::try_from(host_key_type) else {
            *certificate_error.borrow_mut() = Some(ErrorKind::UnknownKeyType.into());
            return Err(git2::Error::new(
                ErrorCode::Certificate,
                ErrorClass::Callback,
                "unknown remote key type",
            ));
        };

        let Some(host) = known_hosts.host(hostname, host_key_type) else {
            *certificate_error.borrow_mut() = Some(
                ErrorKind::UnknownHost {
                    host: Host::new(hostname.to_owned(), host_key_type, host_key),
                }
                .into(),
            );
            return Err(git2::Error::new(
                ErrorCode::Certificate,
                ErrorClass::Callback,
                "unknown host",
            ));
        };

        if host.key.as_ref() != host_key {
            *certificate_error.borrow_mut() = Some(
                ErrorKind::MissmatchRemoteKey {
                    expected: host.key.clone().into_boxed_str(),
                    actual: host_key.into(),
                }
                .into(),
            );
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
    log::info!("{}", msg);

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
    };

    Ok(true)
}

pub use git2::Oid;

#[frb(opaque, mirror(Oid))]
#[derive(Debug, Clone, Copy)]
pub struct _MyOid([u8; 20]);

#[frb(sync)]
#[must_use]
pub fn oid_to_string(oid: &Oid) -> String {
    oid.to_string()
}

// https://github.com/fzyzcjy/flutter_rust_bridge/issues/1937
#[frb(non_opaque)]
pub struct CommitItem {
    pub oid: Oid,
    pub parent: Option<Oid>,
    pub message: String,
    pub author: String,
    pub email: String,
}

#[frb(ignore)]
pub struct LogIter<'repo> {
    repository: &'repo Repository,
    revwalk: git2::Revwalk<'repo>,
}

#[frb(ignore)]
impl Iterator for LogIter<'_> {
    type Item = Result<CommitItem, RustError>;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self
            .revwalk
            .next()?
            .and_then(|oid| self.repository.find_commit(oid))
            .map_err(RustError::from)
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

impl TaskStorage {
    pub fn log(
        &mut self,
        oid: Option<Oid>,
        n: Option<u32>,
    ) -> Result<Option<Vec<CommitItem>>, RustError> {
        let repository = match Repository::open(&self.repository_path) {
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
