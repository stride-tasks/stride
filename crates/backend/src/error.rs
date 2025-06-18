use stride_core::task::TaskStatus;
use stride_crypto::crypter::Error as EncryptionError;

use std::{env::VarError, io::Error as IoError};

use crate::git::known_hosts::{Host, KnownHostsError};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] IoError),

    #[error("key store error: {0}")]
    KeyStore(#[from] KeyStoreError),

    #[error("encryption error: {0}")]
    Encryption(#[from] EncryptionError),

    #[error("corrupted task error")]
    CorruptTask,

    #[error("base64 decryption error: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("libgit2 error: {0}")]
    LibGit2(#[from] git2::Error),

    #[error("known hosts error: {0}")]
    KnownHosts(#[from] KnownHostsError),
    #[error("unknown host error: {host}")]
    UnknownHost { host: Host },
    #[error("missing host key: {hostname}")]
    MissingHostKey { hostname: Box<str> },
    #[error("unknown key type")]
    UnknownKeyType,
    #[error("mismatched host keys expected {expected}, got {actual}")]
    MismatchRemoteKey {
        expected: Box<str>,
        actual: Box<str>,
    },
    #[error("no ssh key is provided")]
    NoSshKeysProvided,

    #[error("environment variable error: {0}")]
    EnvironmentVariable(#[from] VarError),

    #[error("taskchampion error: {0}")]
    TaskChampion(#[from] ::taskchampion::Error),

    #[error("database error: {0}")]
    Database(#[from] stride_database::Error),

    #[error("config error: {0}")]
    Config(#[from] stride_core::backend::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum KeyStoreError {
    #[error("invalid task status in key store: {identifier}")]
    InvalidTaskStatus { identifier: u8 },
    #[error("incorrect cipher length, expected {expected_length}, actual: {actual_length}")]
    InvalidCipherLength {
        expected_length: usize,
        actual_length: usize,
    },
    #[error("already defined key of type {status:?}")]
    DuplicateEntry { status: TaskStatus },
    #[error("cannot lock key store")]
    LockError,
    #[error("encryption key verification")]
    Verification,
    #[error("missing encryption keys")]
    MissingEncryptionKeys,
    #[error("base64 decryption error: {0}")]
    Base64Decode(#[from] base64::DecodeError),
}
