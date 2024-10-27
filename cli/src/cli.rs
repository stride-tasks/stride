#![forbid(missing_docs)]

//! This module defines the command line arguments for the `stride` CLI.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[allow(clippy::module_name_repetitions)]
/// An command line interface for stride
pub struct CliArgs {
    #[command(subcommand)]
    /// The mode to operate in
    pub mode: Mode,
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

    /// Sync the task storage
    Sync,

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
}
