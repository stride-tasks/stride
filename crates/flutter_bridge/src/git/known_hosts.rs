//! Implements (partial) parsing of known hosts file format
//!
//! See file format [here](https://man7.org/linux/man-pages/man8/sshd.8.html#SSH_KNOWN_HOSTS_FILE_FORMAT).

use std::{fmt::Display, path::Path, str::FromStr};

use flutter_rust_bridge::frb;
use git2::cert::SshHostKeyType;
use serde::{Deserialize, Serialize};

// const BUNDELED_KEYS: &[KnownHostRef<'_>] = &[];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostKeyType {
    Rsa = SshHostKeyType::Rsa as isize,
    Dss = SshHostKeyType::Dss as isize,
    Ecdsa256 = SshHostKeyType::Ecdsa256 as isize,
    Ecdsa384 = SshHostKeyType::Ecdsa384 as isize,
    Ecdsa521 = SshHostKeyType::Ecdsa521 as isize,
    Ed255219 = SshHostKeyType::Ed255219 as isize,
}

impl HostKeyType {
    /// The name of the key type as encoded in the `known_hosts` file.
    #[must_use]
    pub fn name(&self) -> &'static str {
        SshHostKeyType::from(*self).name()
    }

    /// A short name of the key type, the colloquial form used as a human-readable description.
    #[must_use]
    pub fn short_name(&self) -> &'static str {
        SshHostKeyType::from(*self).short_name()
    }
}

impl From<HostKeyType> for SshHostKeyType {
    fn from(value: HostKeyType) -> Self {
        match value {
            HostKeyType::Rsa => SshHostKeyType::Rsa,
            HostKeyType::Dss => SshHostKeyType::Dss,
            HostKeyType::Ecdsa256 => SshHostKeyType::Ecdsa256,
            HostKeyType::Ecdsa384 => SshHostKeyType::Ecdsa384,
            HostKeyType::Ecdsa521 => SshHostKeyType::Ecdsa521,
            HostKeyType::Ed255219 => SshHostKeyType::Ed255219,
        }
    }
}

impl TryFrom<SshHostKeyType> for HostKeyType {
    type Error = HostKeyTypeError;
    fn try_from(value: SshHostKeyType) -> Result<Self, Self::Error> {
        match value {
            SshHostKeyType::Rsa => Ok(Self::Rsa),
            SshHostKeyType::Dss => Ok(Self::Dss),
            SshHostKeyType::Ecdsa256 => Ok(Self::Ecdsa256),
            SshHostKeyType::Ecdsa384 => Ok(Self::Ecdsa384),
            SshHostKeyType::Ecdsa521 => Ok(Self::Ecdsa521),
            SshHostKeyType::Ed255219 => Ok(Self::Ed255219),
            _ => Err(HostKeyTypeError::Unknown {
                value: "<NONE>".to_owned(),
            }),
        }
    }
}

impl Display for HostKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.short_name())
    }
}

#[derive(Debug)]
pub enum HostKeyTypeError {
    Unknown { value: String },
}

impl std::error::Error for HostKeyTypeError {}
impl Display for HostKeyTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown { value } => write!(f, "unknown host key type \"{value}\""),
        }
    }
}

impl FromStr for HostKeyType {
    type Err = HostKeyTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ssh-rsa" => Ok(HostKeyType::Rsa),
            "ssh-dss" => Ok(HostKeyType::Dss),
            "ecdsa-sha2-nistp256" => Ok(HostKeyType::Ecdsa256),
            "ecdsa-sha2-nistp384" => Ok(HostKeyType::Ecdsa384),
            "ecdsa-sha2-nistp521" => Ok(HostKeyType::Ecdsa521),
            "ssh-ed25519" => Ok(HostKeyType::Ed255219),
            value => Err(HostKeyTypeError::Unknown {
                value: value.to_owned(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HostRef<'a> {
    pub hostname: &'a str,
    pub key_type: HostKeyType,

    /// The a reference to public key of the remote server/host.
    ///
    /// The key is base64 encoded.
    pub key: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Host {
    pub hostname: String,
    pub key_type: HostKeyType,

    /// The public key of the remote server/host.
    ///
    /// The key is base64 encoded.
    pub key: String,
}

impl Host {
    #[must_use]
    pub fn new(hostname: String, key_type: HostKeyType, key: String) -> Self {
        Self {
            hostname,
            key_type,
            key,
        }
    }

    #[must_use]
    pub fn as_ref(&self) -> HostRef<'_> {
        HostRef {
            hostname: &self.hostname,
            key_type: self.key_type,
            key: &self.key,
        }
    }
}

impl Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.hostname, self.key_type.name(), self.key)
    }
}

#[derive(Debug)]
pub enum HostParseError {
    MissingHostname,
    MissingKeyType,
    InvalidKeyType(HostKeyTypeError),
    MissingRemoteHostKey,
}

impl std::error::Error for HostParseError {}
impl Display for HostParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingHostname => f.write_str("missing hostname"),
            Self::MissingKeyType => f.write_str("missing key type"),
            Self::InvalidKeyType(error) => write!(f, "Invalid key type {error}"),
            Self::MissingRemoteHostKey => f.write_str("missing remote host key"),
        }
    }
}

impl From<HostKeyTypeError> for HostParseError {
    fn from(error: HostKeyTypeError) -> Self {
        Self::InvalidKeyType(error)
    }
}

impl FromStr for Host {
    type Err = HostParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.split_whitespace();

        let hostname = input.next().ok_or(HostParseError::MissingHostname)?;

        let key_type = input
            .next()
            .ok_or(HostParseError::MissingKeyType)?
            .parse()?;

        // TODO: Check if it's valid base64 encoded
        let key = input.next().ok_or(HostParseError::MissingRemoteHostKey)?;

        Ok(Self {
            hostname: hostname.to_owned(),
            key_type,
            key: key.to_owned(),
        })
    }
}

#[derive(Debug)]
pub enum KnownHostsParseError {
    InvalidHost(HostParseError),
}

impl std::error::Error for KnownHostsParseError {}
impl Display for KnownHostsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KnownHostsParseError::InvalidHost(error) => write!(f, "Invalid host {error}"),
        }
    }
}

impl From<HostParseError> for KnownHostsParseError {
    fn from(error: HostParseError) -> Self {
        Self::InvalidHost(error)
    }
}

#[derive(Debug)]
pub enum KnownHostsError {
    InvalidKnownHosts(KnownHostsParseError),
    IoError(std::io::Error),
    VarError(std::env::VarError),
}

impl std::error::Error for KnownHostsError {}
impl Display for KnownHostsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidKnownHosts(error) => {
                write!(f, "Invalid known hosts {error}")
            }
            Self::IoError(error) => {
                write!(f, "Input/output error {error}")
            }
            Self::VarError(error) => {
                write!(f, "environment variable error {error}")
            }
        }
    }
}

impl From<KnownHostsParseError> for KnownHostsError {
    fn from(error: KnownHostsParseError) -> Self {
        Self::InvalidKnownHosts(error)
    }
}
impl From<std::io::Error> for KnownHostsError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
impl From<std::env::VarError> for KnownHostsError {
    fn from(error: std::env::VarError) -> Self {
        Self::VarError(error)
    }
}

#[frb(dart_metadata=("freezed"))]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KnownHosts {
    pub hosts: Vec<Host>,
}

impl KnownHosts {
    const SSH_KNOWN_HOSTS_STANDARD_LOCATION: &'static str = ".ssh/known_hosts";

    #[must_use]
    pub fn new() -> Self {
        Self { hosts: Vec::new() }
    }

    pub fn parse_str(input: &str) -> Result<KnownHosts, KnownHostsParseError> {
        let mut hosts = Vec::new();
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            hosts.push(line.parse()?);
        }

        Ok(Self { hosts })
    }

    pub fn read_file(filepath: &Path) -> Result<KnownHosts, KnownHostsError> {
        let contents = std::fs::read_to_string(filepath)?;
        Ok(Self::parse_str(&contents)?)
    }

    pub fn read_standard_file() -> Result<KnownHosts, KnownHostsError> {
        let home = std::env::var("HOME")?;
        Self::read_file(&Path::new(&home).join(Self::SSH_KNOWN_HOSTS_STANDARD_LOCATION))
    }

    pub fn write_file(&self, filepath: &Path) -> Result<(), std::io::Error> {
        let contents = self.to_string();
        std::fs::write(filepath, contents)?;
        Ok(())
    }

    pub fn write_standard_file(&self) -> Result<(), KnownHostsError> {
        let home = std::env::var("HOME")?;
        self.write_file(&Path::new(&home).join(Self::SSH_KNOWN_HOSTS_STANDARD_LOCATION))?;
        Ok(())
    }

    pub fn add(&mut self, host: Host) {
        self.hosts.push(host);
    }

    #[must_use]
    pub fn hosts(&self) -> &[Host] {
        &self.hosts
    }

    #[must_use]
    pub fn host(&self, hostname: &str, host_key_type: HostKeyType) -> Option<&Host> {
        self.hosts
            .iter()
            .find(|host| host.hostname == hostname && host.key_type == host_key_type)
    }
}

impl Display for KnownHosts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for host in self.hosts() {
            writeln!(f, "{host}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Host, HostKeyType, KnownHosts};

    #[test]
    fn ignore_comments_and_empty_lines() -> anyhow::Result<()> {
        let input = r"
            # GitHub known host
            github.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl

            # GitLab known host
            gitlab.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAfuCHKVTjquxvt6CM6tdG4SLp1Btn/nOeHHE5UOzRdf
        ";

        let known_hosts = KnownHosts::parse_str(input)?;

        let hosts = known_hosts.hosts();

        assert_eq!(hosts.len(), 2);

        assert_eq!(hosts[0].hostname, "github.com");
        assert_eq!(hosts[0].key_type, HostKeyType::Ed255219);
        assert_eq!(
            hosts[0].key,
            "AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl"
        );

        assert_eq!(hosts[1].hostname, "gitlab.com");
        assert_eq!(hosts[1].key_type, HostKeyType::Ed255219);
        assert_eq!(
            hosts[1].key,
            "AAAAC3NzaC1lZDI1NTE5AAAAIAfuCHKVTjquxvt6CM6tdG4SLp1Btn/nOeHHE5UOzRdf"
        );
        Ok(())
    }

    #[test]
    fn valid_known_host() -> anyhow::Result<()> {
        let hostname = "github.com";
        let key_type = HostKeyType::Ed255219;
        let remote_host_key =
            "AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl";
        let known_host: Host =
            format!("{hostname} {} {remote_host_key}", key_type.name()).parse()?;

        assert_eq!(known_host.hostname, hostname);
        assert_eq!(known_host.key_type, key_type);
        assert_eq!(known_host.key, remote_host_key);
        Ok(())
    }

    #[test]
    fn known_hosts_to_string() {
        let mut known_hosts = KnownHosts::new();
        known_hosts.add(Host::new(
            "github.com".to_owned(),
            HostKeyType::Ed255219,
            "AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl".to_owned(),
        ));
        known_hosts.add(Host::new(
            "gitlab.com".to_owned(),
            HostKeyType::Ed255219,
            "AAAAC3NzaC1lZDI1NTE5AAAAIAfuCHKVTjquxvt6CM6tdG4SLp1Btn/nOeHHE5UOzRdf".to_owned(),
        ));

        assert_eq!(
            known_hosts.to_string(),
            String::new()
            + "github.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl\n"
            + "gitlab.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAfuCHKVTjquxvt6CM6tdG4SLp1Btn/nOeHHE5UOzRdf\n"
        );
    }
}
