use crate::git::known_hosts::{KnownHosts, KnownHostsError};
use git2::{CertificateCheckStatus, Cred, ErrorClass, ErrorCode, RemoteCallbacks};
use std::{io::ErrorKind, path::PathBuf};
use thiserror::Error;
use uuid::Uuid;

// #[derive(Error, Debug)]
// pub enum ConnectionError {
//     // #[error("Invalid host {0}")]
//     // InvalidHost(#[from] HostParseError),
//     #[error("loading known hosts error {0}")]
//     LoadKnownHosts(#[from] KnownHostsError),

//     #[error("network error: {message}")]
//     Network { message: String },

//     #[error("ssh authentication error: {message}")]
//     Authentication { message: String },

//     #[error("{message}")]
//     Other { message: String },
// }

// pub fn test_connection(
//     url: &str,
//     public_key: &str,
//     private_key: &str,
// ) -> Result<(), ConnectionError> {
//     let known_hosts = KnownHosts::read_standard_file();
//     if let Err(KnownHostsError::IoError(error)) = known_hosts {
//         if error.kind() == ErrorKind::NotFound {}
//     }

//     let mut tried_ssh = false;

//     let mut callbacks = RemoteCallbacks::new();
//     callbacks.credentials(|_url, username_from_url, allowed_types| {
//         if tried_ssh {
//             return Err(git2::Error::from_str(&format!(
//                 "Already tried to get the auth: {allowed_types:?}"
//             )));
//         }
//         println!("{username_from_url:?}");

//         tried_ssh = true;

//         Cred::ssh_key_from_memory(
//             username_from_url.unwrap(),
//             Some(public_key),
//             private_key,
//             None,
//         )
//     });

//     callbacks.certificate_check(|cert, hostname| {
//         println!(
//             "certificate_check: {:?}",
//             cert.as_hostkey().and_then(|x| x.hostkey())
//         );

//         let cert = cert.as_hostkey().unwrap();

//         // let mut known_hosts = KnownHosts::new();
//         // known_hosts.add(Host::new(
//         //     hostname.to_owned(),
//         //     cert.hostkey_type().unwrap().try_into().unwrap(),
//         //     base64::engine::general_purpose::STANDARD.encode(cert.hostkey().unwrap()),
//         // ));
//         // known_hosts.write_standard_file().unwrap();
//         Result::Ok(CertificateCheckStatus::CertificatePassthrough)
//     });

//     let mut remote = git2::Remote::create_detached(url)?;
//     let connection = remote.connect_auth(git2::Direction::Fetch, Some(callbacks), None);
//     if let Err(error) = &connection {
//         // match error.class() {
//         //     git2::ErrorClass::None => todo!(),
//         //     git2::ErrorClass::NoMemory => todo!(),
//         //     git2::ErrorClass::Os => todo!(),
//         //     git2::ErrorClass::Invalid => todo!(),
//         //     git2::ErrorClass::Reference => todo!(),
//         //     git2::ErrorClass::Zlib => todo!(),
//         //     git2::ErrorClass::Repository => todo!(),
//         //     git2::ErrorClass::Config => todo!(),
//         //     git2::ErrorClass::Regex => todo!(),
//         //     git2::ErrorClass::Odb => todo!(),
//         //     git2::ErrorClass::Index => todo!(),
//         //     git2::ErrorClass::Object => todo!(),
//         //     git2::ErrorClass::Net => todo!(),
//         //     git2::ErrorClass::Tag => todo!(),
//         //     git2::ErrorClass::Tree => todo!(),
//         //     git2::ErrorClass::Indexer => todo!(),
//         //     git2::ErrorClass::Ssl => todo!(),
//         //     git2::ErrorClass::Submodule => todo!(),
//         //     git2::ErrorClass::Thread => todo!(),
//         //     git2::ErrorClass::Stash => todo!(),
//         //     git2::ErrorClass::Checkout => todo!(),
//         //     git2::ErrorClass::FetchHead => todo!(),
//         //     git2::ErrorClass::Merge => todo!(),
//         //     git2::ErrorClass::Ssh => todo!(),
//         //     git2::ErrorClass::Filter => todo!(),
//         //     git2::ErrorClass::Revert => todo!(),
//         //     git2::ErrorClass::Callback => todo!(),
//         //     git2::ErrorClass::CherryPick => todo!(),
//         //     git2::ErrorClass::Describe => todo!(),
//         //     git2::ErrorClass::Rebase => todo!(),
//         //     git2::ErrorClass::Filesystem => todo!(),
//         //     git2::ErrorClass::Patch => todo!(),
//         //     git2::ErrorClass::Worktree => todo!(),
//         //     git2::ErrorClass::Sha1 => todo!(),
//         //     git2::ErrorClass::Http => todo!(),
//         // }
//         // match error.code() {
//         //     ErrorCode::GenericError => todo!(),
//         //     ErrorCode::NotFound => todo!(),
//         //     ErrorCode::Exists => todo!(),
//         //     ErrorCode::Ambiguous => todo!(),
//         //     ErrorCode::BufSize => todo!(),
//         //     ErrorCode::User => todo!(),
//         //     ErrorCode::BareRepo => todo!(),
//         //     ErrorCode::UnbornBranch => todo!(),
//         //     ErrorCode::Unmerged => todo!(),
//         //     ErrorCode::NotFastForward => todo!(),
//         //     ErrorCode::InvalidSpec => todo!(),
//         //     ErrorCode::Conflict => todo!(),
//         //     ErrorCode::Locked => todo!(),
//         //     ErrorCode::Modified => todo!(),
//         //     ErrorCode::Auth => todo!(),
//         //     ErrorCode::Certificate => todo!(),
//         //     ErrorCode::Applied => todo!(),
//         //     ErrorCode::Peel => todo!(),
//         //     ErrorCode::Eof => todo!(),
//         //     ErrorCode::Invalid => todo!(),
//         //     ErrorCode::Uncommitted => todo!(),
//         //     ErrorCode::Directory => todo!(),
//         //     ErrorCode::MergeConflict => todo!(),
//         //     ErrorCode::HashsumMismatch => todo!(),
//         //     ErrorCode::IndexDirty => todo!(),
//         //     ErrorCode::ApplyFail => todo!(),
//         //     ErrorCode::Owner => todo!(),
//         // }
//         return match error.class() {
//             ErrorClass::Ssh => Err(ConnectionError::Authentication {
//                 message: error.message().to_owned(),
//             }),
//             ErrorClass::Net => Err(ConnectionError::Network {
//                 message: error.message().to_owned(),
//             }),
//             _ => todo!(),
//         };
//     }

//     connection?;

//     Ok(())
// }

pub struct GitSource {
    /// Location of the repository.
    path: PathBuf,
}

impl GitSource {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn add_files(&self) {}

    pub fn commit(&self) {}

    pub fn push(&self) {}
}
