#![allow(unused_crate_dependencies)]

mod common;

use common::*;
use stride_flutter_bridge::api::repository::TaskStorage;

#[test]
fn init_repository() -> anyhow::Result<()> {
    let mut fixture = setup("init_repository")?;

    let mut storage = TaskStorage::new(&fixture.support_dir.join("repository").to_string_lossy());

    let commits = storage.log(None, None)?;
    assert!(commits.is_none());

    storage.init_repotitory()?;

    let commits = storage.log(None, None)?;
    let commits = commits.unwrap();

    assert_eq!(commits.len(), 1);
    assert_eq!(commits[0].message, "Initial Commit");

    fixture.cleanup();
    Ok(())
}
