use flutter_rust_bridge::frb;
use git2::Repository;

use crate::{api::repository::TaskStorage, RustError};

#[frb(ignore)]
pub struct LogIter<'repo> {
    repository: &'repo Repository,
    revwalk: git2::Revwalk<'repo>,
}

#[frb(ignore)]
pub struct CommitItem {
    pub oid: git2::Oid,
    pub parent: Option<git2::Oid>,
    pub message: Box<str>,
    pub author: Box<str>,
    pub email: Box<str>,
}

impl Iterator for LogIter<'_> {
    type Item = Result<CommitItem, RustError>;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self
            .revwalk
            .next()?
            .and_then(|oid| self.repository.find_commit(oid))
            .map_err(RustError::from)
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

impl TaskStorage {
    #[frb(ignore)]
    pub fn log(
        &mut self,
        oid: Option<git2::Oid>,
        n: Option<usize>,
    ) -> Result<Option<Vec<CommitItem>>, RustError> {
        let repository = match Repository::open(&self.repository_path) {
            Ok(repository) => repository,
            Err(error) if error.code() == git2::ErrorCode::NotFound => return Ok(None),
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
        .take(n.unwrap_or(usize::MAX))
        {
            commits.push(commit?);
        }

        Ok(Some(commits))
    }
}
