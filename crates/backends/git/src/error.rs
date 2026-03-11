use stride_backend::error::BackendError;
use stride_crdt::{actor::ActorId, change::Sequence};
use stride_crypto::crypter::Error as EncryptionError;

use std::{env::VarError, io::Error as IoError};

use crate::{
    known_hosts::{Host, KnownHostsError},
    ssh_key::SshError,
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] IoError),

    #[error("encryption error: {0}")]
    Encryption(#[from] EncryptionError),

    #[error("base64 decryption error: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("libgit2 error: {0}")]
    LibGit2(#[from] git2::Error),

    #[error("unsupported changlog version {version} from actor: {actor_id}")]
    UnsupportedVersion { actor_id: ActorId, version: u8 },

    #[error("changelog actor mismatch expected: {change_log_actor_id}, got: {change_actor_id}")]
    ActorMissmatch {
        change_log_actor_id: ActorId,
        change_actor_id: ActorId,
    },

    #[error(
        "applying chnages out of order in actor {actor_id} changelog, expected sequence: {expected_sequence}, got {actual_sequence}"
    )]
    ApplyingChangeOutOfOrder {
        actor_id: ActorId,
        expected_sequence: Sequence,
        actual_sequence: Sequence,
    },

    #[error("serialization error: {0}")]
    Serialization(#[from] stride_serialize::Error),

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

    #[error("database error: {0}")]
    Database(#[from] stride_database::Error),

    #[error("config error: {0}")]
    Config(#[from] stride_core::backend::Error),

    #[error("ssh error: {0}")]
    Ssh(#[from] SshError),
}

impl BackendError for Error {}
