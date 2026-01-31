use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use stride_flutter_bridge::api::settings::Settings;

#[allow(dead_code)]
pub(crate) struct TestFixture {
    pub(crate) path: PathBuf,
    pub(crate) support_dir: PathBuf,
    pub(crate) cache_dir: PathBuf,
    pub(crate) document_dir: PathBuf,
}

#[allow(dead_code)]
impl TestFixture {
    pub(crate) fn cleanup(&mut self) {
        std::fs::remove_dir_all(&self.path).expect("the fixture directory should exist");
    }

    pub(crate) fn scoped(self) -> TestFixtureScoped {
        self.into()
    }
}

pub(crate) struct TestFixtureScoped {
    fixture: TestFixture,
}

impl Drop for TestFixtureScoped {
    fn drop(&mut self) {
        self.cleanup();
    }
}

impl From<TestFixture> for TestFixtureScoped {
    fn from(fixture: TestFixture) -> Self {
        Self { fixture }
    }
}

impl Deref for TestFixtureScoped {
    type Target = TestFixture;
    fn deref(&self) -> &Self::Target {
        &self.fixture
    }
}
impl DerefMut for TestFixtureScoped {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fixture
    }
}

#[allow(unused)]
pub(crate) fn setup(test_name: &str) -> Result<TestFixture, std::io::Error> {
    let test_dir = std::env::temp_dir().join("stride_tests").join(test_name);
    if test_dir.exists() {
        eprintln!(
            "[ERROR]: {} test directory already exists",
            test_dir.display()
        );
    }

    std::fs::create_dir_all(&test_dir)?;

    let support_dir = test_dir.join("support");
    let cache_dir = test_dir.join("cache");
    let document_dir = test_dir.join("document");

    std::fs::create_dir(&support_dir)?;
    std::fs::create_dir(&cache_dir)?;
    std::fs::create_dir(&document_dir)?;

    Settings::load(stride_flutter_bridge::api::settings::ApplicationPaths {
        cache_path: cache_dir.clone().to_string_lossy().to_string(),
        support_path: support_dir.clone().to_string_lossy().to_string(),
        log_path: cache_dir
            .join("logs")
            .join("log.txt")
            .to_string_lossy()
            .to_string(),
    })
    .unwrap();

    Ok(TestFixture {
        path: test_dir,
        support_dir,
        cache_dir,
        document_dir,
    })
}
