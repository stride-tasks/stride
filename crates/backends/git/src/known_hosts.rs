//! Implements (partial) parsing of known hosts file format
//!
//! See file format [here](https://man7.org/linux/man-pages/man8/sshd.8.html#SSH_KNOWN_HOSTS_FILE_FORMAT).

use std::{fmt::Display, path::Path, str::FromStr};

use git2::cert::SshHostKeyType;
use serde::{Deserialize, Serialize};

use crate::Result;

// const BUNDELED_KEYS: &[KnownHostRef<'_>] = &[];

/// flutter_rust_bridge:ignore
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
    ///
    /// flutter_rust_bridge:ignore
    #[must_use]
    pub fn name(&self) -> &'static str {
        SshHostKeyType::from(*self).name()
    }

    /// A short name of the key type, the colloquial form used as a human-readable description.
    ///
    /// flutter_rust_bridge:ignore
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

/// flutter_rust_bridge:ignore
impl TryFrom<SshHostKeyType> for HostKeyType {
    type Error = KnownHostsError;
    fn try_from(value: SshHostKeyType) -> Result<Self, Self::Error> {
        match value {
            SshHostKeyType::Rsa => Ok(Self::Rsa),
            SshHostKeyType::Dss => Ok(Self::Dss),
            SshHostKeyType::Ecdsa256 => Ok(Self::Ecdsa256),
            SshHostKeyType::Ecdsa384 => Ok(Self::Ecdsa384),
            SshHostKeyType::Ecdsa521 => Ok(Self::Ecdsa521),
            SshHostKeyType::Ed255219 => Ok(Self::Ed255219),
            _ => Err(KnownHostsError::UnknownHostKeyType),
        }
    }
}

impl Display for HostKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.short_name())
    }
}

/// flutter_rust_bridge:ignore
impl FromStr for HostKeyType {
    type Err = KnownHostsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ssh-rsa" => Ok(HostKeyType::Rsa),
            "ssh-dss" => Ok(HostKeyType::Dss),
            "ecdsa-sha2-nistp256" => Ok(HostKeyType::Ecdsa256),
            "ecdsa-sha2-nistp384" => Ok(HostKeyType::Ecdsa384),
            "ecdsa-sha2-nistp521" => Ok(HostKeyType::Ecdsa521),
            "ssh-ed25519" => Ok(HostKeyType::Ed255219),
            _ => Err(KnownHostsError::UnknownHostKeyType),
        }
    }
}

/// flutter_rust_bridge:ignore
#[derive(Debug, Clone, Copy)]
pub struct HostRef<'a> {
    pub hostname: &'a str,
    pub key_type: HostKeyType,

    /// The a reference to public key of the remote server/host.
    ///
    /// The key is base64 encoded.
    pub key: &'a str,
}

/// flutter_rust_bridge:non_opaque
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
    /// flutter_rust_bridge:ignore
    #[must_use]
    pub fn new(hostname: String, key_type: HostKeyType, key: String) -> Self {
        Self {
            hostname,
            key_type,
            key,
        }
    }

    /// flutter_rust_bridge:ignore
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

/// flutter_rust_bridge:ignore
impl FromStr for Host {
    type Err = KnownHostsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.split_whitespace();

        let hostname = input.next().ok_or(KnownHostsError::MissingHostname)?;

        let key_type = input
            .next()
            .ok_or(KnownHostsError::MissingKeyType)?
            .parse()?;

        // TODO: Check if it's valid base64 encoded
        let key = input.next().ok_or(KnownHostsError::MissingRemoteHostKey)?;

        Ok(Self {
            hostname: hostname.to_owned(),
            key_type,
            key: key.to_owned(),
        })
    }
}

/// flutter_rust_bridge:ignore
#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum KnownHostsError {
    #[error("missing hostname")]
    MissingHostname,
    #[error("missing key type")]
    MissingKeyType,
    #[error("missing remote host key")]
    MissingRemoteHostKey,
    #[error("unknown remote host key")]
    UnknownHostKeyType,
}

/// flutter_rust_bridge:non_opaque
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KnownHosts {
    pub hosts: Vec<Host>,
}

impl KnownHosts {
    const SSH_KNOWN_HOSTS_STANDARD_LOCATION: &'static str = ".ssh/known_hosts";

    pub fn load() -> Result<Self> {
        KnownHosts::read_standard_file()
    }

    pub fn save(this: &Self) -> Result<()> {
        KnownHosts::write_standard_file(this)
    }

    /// flutter_rust_bridge:ignore
    #[must_use]
    pub fn new() -> Self {
        Self { hosts: Vec::new() }
    }

    /// flutter_rust_bridge:ignore
    pub fn parse_str(input: &str) -> Result<Self> {
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

    /// flutter_rust_bridge:ignore
    pub fn read_file(filepath: &Path) -> Result<Self> {
        if !filepath.exists() {
            std::fs::write(filepath, "")?;
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(filepath)?;
        Self::parse_str(&contents)
    }

    /// flutter_rust_bridge:ignore
    pub fn read_standard_file() -> Result<Self> {
        let home = std::env::var("HOME")?;
        Self::read_file(&Path::new(&home).join(Self::SSH_KNOWN_HOSTS_STANDARD_LOCATION))
    }

    /// flutter_rust_bridge:ignore
    pub fn write_file(&self, filepath: &Path) -> Result<(), std::io::Error> {
        let contents = self.to_string();
        std::fs::write(filepath, contents)?;
        Ok(())
    }

    /// flutter_rust_bridge:ignore
    pub fn write_standard_file(&self) -> Result<()> {
        let home = std::env::var("HOME")?;
        self.write_file(&Path::new(&home).join(Self::SSH_KNOWN_HOSTS_STANDARD_LOCATION))?;
        Ok(())
    }

    /// flutter_rust_bridge:ignore
    pub fn add(&mut self, host: Host) {
        self.hosts.push(host);
    }

    /// flutter_rust_bridge:ignore
    pub fn remove_by_hostname(&mut self, hostname: &str) {
        self.hosts.retain(|host| host.hostname != hostname);
    }

    /// flutter_rust_bridge:ignore
    #[must_use]
    pub fn hosts(&self) -> &[Host] {
        &self.hosts
    }

    /// flutter_rust_bridge:ignore
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
