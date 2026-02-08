use std::{
    collections::HashMap,
    iter::FusedIterator,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use git2::{DiffFormat, DiffOptions, Repository};
use stride_core::task::TaskStatus;
use stride_database::operation::{Operation, difference::push_operations_diff_task_with_timestamp};

use crate::{Result, key_store::KeyStore};

fn git2_time_to_chrono(git_time: git2::Time) -> DateTime<Utc> {
    // Get the seconds and offset from the git2::Time object
    let seconds = git_time.seconds();
    DateTime::from_timestamp_secs(seconds).expect("should not overflow")
}

enum Change {
    None,
    Pending,
    Some(Operation),
}

pub(crate) struct OperationIterator<'a> {
    repository: &'a Repository,
    revwalk: git2::Revwalk<'a>,
    key_store: &'a KeyStore,
    operations: Vec<Operation>,
}

impl<'a> OperationIterator<'a> {
    pub(crate) fn new(repository: &'a Repository, key_store: &'a KeyStore) -> Result<Self> {
        let mut revwalk = repository.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME | git2::Sort::REVERSE)?;

        Ok(Self {
            repository,
            revwalk,
            key_store,
            operations: Vec::new(),
        })
    }

    fn next_inner(&mut self) -> Result<Change> {
        pub(crate) struct TaskDiff {
            path: PathBuf,
            adding: bool,
            // TODO: content no longer has to be a String, it can be a &[u8]
            content: String,
        }

        if let Some(operation) = self.operations.pop() {
            return Ok(Change::Some(operation));
        }

        let Some(oid) = self.revwalk.next().transpose()? else {
            return Ok(Change::None);
        };
        let commit = self.repository.find_commit(oid)?;

        let tree = commit.tree()?;
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let mut diff_opts = DiffOptions::new();
        diff_opts.pathspec("tasks/"); // directory to scope the diff to.
        diff_opts.context_lines(0);
        diff_opts.ignore_whitespace(true);
        diff_opts.ignore_whitespace_eol(true);

        let diff = self.repository.diff_tree_to_tree(
            parent_tree.as_ref(),
            Some(&tree),
            Some(&mut diff_opts),
        )?;

        let mut diffs = Vec::new();
        diff.print(DiffFormat::Patch, |delta, _hunk, line| {
            if !matches!(
                line.origin_value(),
                git2::DiffLineType::Addition | git2::DiffLineType::Deletion
            ) {
                return true;
            }
            let path = delta.new_file().path().unwrap();
            let content = String::from_utf8_lossy(line.content().trim_ascii_end());
            // println!("AT:{} {} {content}", path.display(), line.origin());

            diffs.push(TaskDiff {
                path: path.to_path_buf(),
                adding: line.origin_value() == git2::DiffLineType::Addition,
                content: content.into_owned(),
            });

            true
        })?;

        let timestamp = git2_time_to_chrono(commit.committer().when());

        let mut tasks = HashMap::new();
        for TaskDiff {
            path,
            adding,
            content,
        } in diffs
        {
            let status = if path == Path::new("tasks/pending") {
                TaskStatus::Pending
            } else if path == Path::new("tasks/complete") {
                TaskStatus::Complete
            } else if path == Path::new("tasks/deleted") {
                TaskStatus::Deleted
            } else if path == Path::new("tasks/keys") {
                continue;
            } else {
                log::warn!("skipping unknown path: {}", path.display());
                continue;
            };
            let (_, task) = self.key_store.decrypt(status, &content)?;

            let entry = tasks.entry(task.uuid).or_insert((None, None));
            if adding {
                entry.0 = Some(task);
            } else {
                entry.1 = Some(task);
            }
        }

        for (current, previous) in tasks.into_values() {
            push_operations_diff_task_with_timestamp(
                current.as_ref(),
                previous.as_ref(),
                timestamp,
                &mut self.operations,
            );
        }

        Ok(Change::Pending)
    }
}

impl Iterator for OperationIterator<'_> {
    type Item = Result<Operation>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.next_inner() {
                Ok(Change::Pending) => {}
                Ok(Change::None) => break None,
                Ok(Change::Some(operation)) => break Some(Ok(operation)),
                Err(err) => break Some(Err(err)),
            }
        }
    }
}

impl FusedIterator for OperationIterator<'_> {}
