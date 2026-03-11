//! Stride's backend implementations.

#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use base64::{DecodeError, Engine};
use config::GitConfig;
use git2::{
    AnnotatedCommit, Branch, CertificateCheckStatus, Cred, ErrorClass, ErrorCode, FetchOptions,
    Oid, Reference, RemoteCallbacks, Repository, ResetType, Signature, StatusOptions,
    build::CheckoutBuilder,
};
use known_hosts::{Host, HostKeyType, KnownHosts};
use ssh_key::SshKey;
use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Lines, Seek, Write},
    iter::{FusedIterator, Skip},
    path::Path,
    rc::Rc,
    sync::Arc,
};
use stride_backend::{Backend, BackendHandler};
use stride_crdt::{
    actor::{Actor, ActorId},
    change::{Change, Sequence},
    hlc::{Microsecond, Timestamp},
    version_vector::{ChangeLocation, VersionVector},
};
use stride_crypto::crypter::Crypter;
use stride_database::Database;
use uuid::Uuid;

mod serialization;

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

/// flutter_rust_bridge:ignore
pub mod config;

pub mod known_hosts;
pub mod ssh_key;

use crate::{
    config::Handler,
    serialization::{change_from_data, change_to_data},
};

struct ChangeIter<'a> {
    actor_id: ActorId,
    lines: Skip<Lines<BufReader<&'a mut File>>>,
    key: &'a Crypter,
}
impl ChangeIter<'_> {
    fn next_impl(&mut self) -> Result<Option<Change>> {
        let Some(change) = self.lines.next().transpose()? else {
            return Ok(None);
        };

        if change.is_empty() {
            unreachable!("should not contain any empty newlines");
        }

        let change = base64_decode(change)?;
        let (_, _, data) = self.key.decrypt(&change, 0)?;
        Ok(Some(change_from_data(self.actor_id, &data)?))
    }
}

impl Iterator for ChangeIter<'_> {
    type Item = Result<Change>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_impl().transpose()
    }
}

impl FusedIterator for ChangeIter<'_> {}

/// Actor change log storage
struct ActorStorage {
    actor_id: ActorId,
    sequence: Sequence,
    file: File,
    master_key: Arc<Crypter>,
    key: Crypter,
}

impl ActorStorage {
    const KEY_VERSION_LEN: usize = 1;
    const KEY_SEQUENCE_LEN: usize = size_of::<u64>();
    const KEY_AAD_LEN: usize = Self::KEY_VERSION_LEN + Self::KEY_SEQUENCE_LEN;

    fn load(
        actor_id: ActorId,
        changelog_filepath: &Path,
        master_key: Arc<Crypter>,
    ) -> Result<Option<Self>> {
        if !changelog_filepath.exists() {
            return Ok(None);
        }

        let file = File::options()
            .read(true)
            .write(true)
            .open(changelog_filepath)?;
        let mut reader = BufReader::new(file);

        let mut key = String::new();
        reader.read_line(&mut key)?;

        // NOTE: read_line includes the newline as well.
        let key = key.trim();

        if key.is_empty() {
            return Ok(None);
        }

        let key = base64::engine::general_purpose::URL_SAFE.decode(key)?;

        let (aad, _, data) = master_key.decrypt(&key, Self::KEY_AAD_LEN)?;

        let ([version], aad) = aad
            .split_first_chunk::<{ Self::KEY_VERSION_LEN }>()
            .expect("shoud contain version");
        if *version != 0 {
            return Err(Error::UnsupportedVersion {
                actor_id,
                version: *version,
            });
        }

        let (sequence_bytes, aad) = aad
            .split_first_chunk::<{ Self::KEY_SEQUENCE_LEN }>()
            .expect("shoud contain sequence");
        let sequence = Sequence::new(u64::from_be_bytes(*sequence_bytes));

        assert_eq!(aad.len(), 0, "should not contain any remaining data");

        let key = Crypter::new(data.try_into().unwrap());

        Ok(Some(Self {
            actor_id,
            sequence,
            file: reader.into_inner(),
            master_key,
            key,
        }))
    }

    fn create(
        actor_id: ActorId,
        changelog_filepath: &Path,
        master_key: Arc<Crypter>,
    ) -> Result<Self> {
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(changelog_filepath)?;

        let sequence = Sequence::new(0);

        let mut aad = Vec::new();
        aad.push(0);
        aad.extend_from_slice(&sequence.get().to_be_bytes());
        assert_eq!(aad.len(), Self::KEY_AAD_LEN);

        let key = Crypter::generate();
        let key_encrypted = master_key.encrypt(key.encryption_key(), &aad)?;
        let key_base64 = base64::engine::general_purpose::URL_SAFE.encode(key_encrypted);

        file.write_all(key_base64.as_bytes())?;

        Ok(Self {
            actor_id,
            sequence,
            file,
            master_key,
            key,
        })
    }

    #[allow(clippy::iter_not_returning_iterator)]
    fn iter(&mut self, skip: usize) -> Result<ChangeIter<'_>> {
        self.file.seek(std::io::SeekFrom::Start(0))?;
        Ok(ChangeIter {
            actor_id: self.actor_id,
            // Note: +1 for encrypted key.
            lines: BufReader::new(&mut self.file).lines().skip(skip + 1),
            key: &self.key,
        })
    }

    fn append(&mut self, changes: &[Change]) -> Result<()> {
        self.file.seek(std::io::SeekFrom::End(0))?;

        let mut writer = BufWriter::new(&mut self.file);
        for change in changes {
            if change.actor_id != self.actor_id {
                return Err(Error::ActorMissmatch {
                    change_log_actor_id: self.actor_id,
                    change_actor_id: change.actor_id,
                });
            }
            if change.sequence.get() != self.sequence.get() + 1 {
                return Err(Error::ApplyingChangeOutOfOrder {
                    actor_id: self.actor_id,
                    expected_sequence: Sequence::new(self.sequence.get() + 1),
                    actual_sequence: change.sequence,
                });
            }

            let mut blob = Vec::new();
            change_to_data(change, &mut blob);

            let change_encrypted = self.key.encrypt(&blob, &[])?;
            let change_base64 = base64_encode(change_encrypted);

            writer.write_all(b"\n")?;
            writer.write_all(change_base64.as_bytes())?;

            self.sequence = Sequence::new(self.sequence.get() + 1);
        }

        writer.seek(std::io::SeekFrom::Start(0))?;

        {
            let mut aad = Vec::new();
            aad.push(0);
            aad.extend_from_slice(&self.sequence.get().to_be_bytes());

            assert_eq!(aad.len(), Self::KEY_AAD_LEN);

            let key_encrypted = self.master_key.encrypt(self.key.encryption_key(), &aad)?;
            let key_base64 = base64::engine::general_purpose::URL_SAFE.encode(key_encrypted);

            writer.write_all(key_base64.as_bytes())?;
        }

        writer.flush()?;
        Ok(())
    }
}

#[allow(missing_debug_implementations)]
pub struct GitBackend {
    config: GitConfig,
    master_key: Arc<Crypter>,
}

impl GitBackend {
    pub fn new(config: GitConfig) -> Result<Self> {
        let repository_path = config.repository_path();

        std::fs::create_dir_all(&repository_path)?;

        let key = config.encryption_key.as_ref();
        let crypter = Arc::new(Crypter::new(key.try_into().unwrap()));

        Ok(Self {
            config,
            master_key: crypter,
        })
    }

    fn clone_repository(&mut self) -> Result<()> {
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
        Ok(())
    }

    fn init_repotitory<'a>(&self, repository: &'a Repository) -> Result<Branch<'a>> {
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

    fn add_and_commit(&self, message: &str) -> Result<bool> {
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

    fn pull(&mut self, repository: &Repository) -> Result<()> {
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

        log::trace!("git sync ahead: {ahead}, behind: {behind}");

        if ahead != 0 {
            return self.reset(repository, &remote);
        }

        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true); // Ensure we see new files

        let statuses = repository.statuses(Some(&mut status_opts))?;

        if !statuses.is_empty() {
            return self.reset(repository, &remote);
        }

        if behind == 0 {
            return Ok(());
        }

        let fetch_head = repository.find_reference("FETCH_HEAD")?;
        let fetch_commit = repository.reference_to_annotated_commit(&fetch_head)?;

        let remote_branch = &self.config.branch;
        do_merge(repository, remote_branch, &fetch_commit)?;

        Ok(())
    }

    fn push(&self, repository: &Repository, force: bool) -> Result<()> {
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

    fn reset(&mut self, repository: &Repository, remote: &Reference<'_>) -> Result<()> {
        // Checkout main
        let branch = &self.config.branch;
        repository.set_head(&format!("refs/heads/{branch}"))?;
        repository.checkout_head(None)?;

        let mut checkout = CheckoutBuilder::new();
        checkout.force();
        checkout.remove_untracked(true);

        // Hard reset main to origin/main
        repository.reset(
            &remote.peel(git2::ObjectType::Commit)?,
            ResetType::Hard,
            Some(&mut checkout),
        )?;
        Ok(())
    }

    fn sync_impl(&mut self, db: &mut Database) -> Result<()> {
        let repository_path = self.config.repository_path();
        let mut cloned = false;
        if !repository_path.join(".git").exists() {
            log::info!("Cloning repository...");
            self.clone_repository()?;
            cloned = true;
        }

        let repository = Repository::open(repository_path)?;

        if !cloned {
            self.pull(&repository)?;
        }

        let actors_dir = self.config.actors_path();
        std::fs::create_dir_all(&actors_dir)?;

        let mut remote_version_vector = VersionVector::default();

        let mut transaction = db.transaction()?;
        for actor_path in std::fs::read_dir(&actors_dir)? {
            let actor_path = actor_path?;
            let metadata = actor_path.metadata()?;
            if !metadata.file_type().is_dir() {
                continue;
            }
            let Ok(filename) = actor_path.file_name().into_string() else {
                continue;
            };
            let Ok(id) = Uuid::parse_str(&filename) else {
                continue;
            };
            let actor_id = ActorId::new(id);

            let changelog_filepath = actors_dir
                .join(actor_id.get().to_string())
                .join("changelog");

            if let Some(actor_storage) =
                ActorStorage::load(actor_id, &changelog_filepath, self.master_key.clone())?
            {
                remote_version_vector.insert(Actor {
                    id: actor_storage.actor_id,
                    sequence: actor_storage.sequence,
                    timestamp: Timestamp::new(Microsecond::new(0), 0),
                });
            }
        }

        let diff = transaction.version_vector().merge(&remote_version_vector);

        log::trace!("Version Difference: {diff:#?}");
        for (actor_id, change_range) in diff {
            let actor_id_dir = actors_dir.join(actor_id.get().to_string());
            let changelog_filepath = actor_id_dir.join("changelog");
            match change_range.location {
                ChangeLocation::Local => {
                    let mut actor_storage = if let Some(actor_storage) =
                        ActorStorage::load(actor_id, &changelog_filepath, self.master_key.clone())?
                    {
                        actor_storage
                    } else {
                        std::fs::create_dir_all(&actor_id_dir)?;
                        ActorStorage::create(
                            actor_id,
                            &changelog_filepath,
                            self.master_key.clone(),
                        )?
                    };

                    let changes = transaction.changes(actor_id, change_range)?;
                    actor_storage.append(&changes)?;

                    self.add_and_commit(&format!(
                        "${} ({},{})",
                        actor_id.get(),
                        change_range.from,
                        change_range.from + change_range.count
                    ))?;
                }
                ChangeLocation::Remote => {
                    let Some(mut actor_storage) =
                        ActorStorage::load(actor_id, &changelog_filepath, self.master_key.clone())?
                    else {
                        unreachable!("");
                    };

                    transaction.get_or_insert_actor(actor_id)?;

                    #[allow(clippy::cast_possible_truncation)]
                    for change in actor_storage
                        .iter(change_range.from as usize)?
                        .take(change_range.count as usize)
                    {
                        let change = change?;
                        transaction.apply_change(&change)?;
                    }
                }
            }
        }
        transaction.commit()?;

        log::info!("Pushing tasks...");
        self.push(&repository, false)?;

        log::info!("Task sync finished!");
        Ok(())
    }
}

impl Backend for GitBackend {
    fn handler() -> Box<dyn BackendHandler>
    where
        Self: Sized,
    {
        Box::new(Handler)
    }

    #[allow(clippy::too_many_lines)]
    fn sync(&mut self, db: &mut Database) -> Result<(), stride_backend::Error> {
        Ok(self.sync_impl(db)?)
    }
}
impl GitBackend {
    pub fn clear(&mut self) -> Result<()> {
        std::fs::remove_dir_all(self.config.repository_path())?;
        Ok(())
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
    lb: &mut Reference<'_>,
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
