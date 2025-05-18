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
    Undo,

    /// Sync the task storage
    Sync {
        /// Choose backend to sync.
        ///
        /// If `None` then all backends are choosen.
        backend: Backend,
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

    /// Manage plugins.
    Plugin {
        #[command(subcommand)]
        /// Plugin command to apply.
        command: Option<PluginCommand>,
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
