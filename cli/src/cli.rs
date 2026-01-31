#![forbid(missing_docs)]

//! This module defines the command line arguments for the `stride` CLI.

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[allow(clippy::module_name_repetitions)]
/// An command line interface for stride
pub struct CliArgs {
    #[command(subcommand)]
    /// The mode to operate in
    pub mode: Mode,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
/// The possible backend Repository types
pub enum Backend {
    /// A `git` based repository, which tracks changes as commits
    Git,

    /// A `taskchampion` Replica, which can sync to `taskwarrior`
    TaskChampion,
}

#[derive(Debug, Subcommand)]
/// The Modus of operation
pub enum Mode {
    /// Find all task matching the filter
    Search {
        /// A string (or optionally multiple strings concatenated with a space) to search for in the
        /// task list.
        filter: Vec<String>,
    },

    /// Add a task to the storage
    Add {
        /// A description of the task.
        /// If this is equal to '-' we try to read it from `stdin`.
        content: Vec<String>,
    },

    /// Undo previous change.
    Undo {
        /// Undo `count` changes.
        count: Option<usize>,
    },

    /// Sync the task storage
    Sync {
        /// Choose backend to sync.
        backend: String,
    },

    /// Output the git-log of the task storage
    Log {
        /// The number of items to display
        #[arg(default_value_t = u32::MAX)]
        limit: u32,

        /// The number of items to skip
        skip: Option<u32>,
    },

    /// Export the task storage in a task format
    Export {
        /// The output file path. Otherwise will write to `stdout`.
        filepath: Option<PathBuf>,
    },

    /// Import previously `export`ed tasks.
    Import {
        /// The file path to read from. Otherwise will try to read from `stdin`.
        filepath: Option<PathBuf>,
    },

    /// Change repository.
    Repository {
        /// The UUID of the repository.
        uuid: Uuid,
    },

    /// Manage backends.
    Backend {
        /// Backend command to apply.
        #[command(subcommand)]
        command: Option<BackendCommand>,
    },

    /// Manage plugins.
    Plugin {
        /// Plugin command to apply.
        #[command(subcommand)]
        command: Option<PluginCommand>,
    },

    /// Manage SSH options.
    Ssh {
        /// SSH command to apply.
        #[command(subcommand)]
        command: SshCommand,
    },
}

/// Backend command/action to apply.
#[derive(Debug, Clone, Subcommand)]
pub enum BackendCommand {
    /// list backends
    List,
    /// Create a new backend.
    New {
        /// The name of the backend to be created.
        backend_name: String,
    },
    /// Remove a backend backend.
    Remove {
        /// The name of the backend to be removed.
        backend_name: String,
    },
    /// Configure a backend option.
    Config {
        /// Name of backend to configure.
        backend_name: String,

        /// Property name.
        property_name: Option<String>,

        /// Unset a property.
        #[clap(long, conflicts_with = "property_value")]
        unset: bool,

        /// Property name.
        property_value: Option<String>,
    },
    /// Invoke an operation that the backend provides
    Invoke {
        /// Name of backend to invoke a method.
        backend_name: String,

        /// Method Id.
        method_id: Option<String>,
    },
}

/// Plugin command/action to apply.
#[derive(Debug, Clone, Subcommand)]
pub enum PluginCommand {
    /// Import plugin.
    Import {
        /// Filepath to the .zip plugin archive.
        filepath: PathBuf,
    },
    /// Enable/Disable plugin.
    Toggle {
        /// Plugin to toggle.
        plugin_name: String,
    },
}

/// SSH command/action to apply.
#[derive(Debug, Clone, Subcommand)]
pub enum SshCommand {
    /// Manage SSH keys.
    Key {
        /// SSH Key command to apply.
        #[command(subcommand)]
        command: Option<SshKeyCommand>,
    },

    /// Manage SSH known hosts.
    KnownHosts {
        /// SSH Known host command to apply.
        #[command(subcommand)]
        command: Option<SshKnownHostsCommand>,
    },
}

/// SSH command/action to apply.
#[derive(Debug, Clone, Copy, Subcommand)]
pub enum SshKeyCommand {
    /// Generate new SSH key.
    Generate,

    /// Remove a SSH key.
    Remove {
        /// ID of the SSH key to be removed.
        id: Uuid,
    },
}

/// SSH Known host command to apply.
#[derive(Debug, Clone, Subcommand)]
pub enum SshKnownHostsCommand {
    /// Remove a SSH known host.
    Remove {
        /// A Host name (e.g. `github.com`).
        hostname: String,
    },
}
