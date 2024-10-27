use std::collections::HashSet;

use flutter_rust_bridge::frb;
use uuid::Uuid;

use crate::{
    task::{Task, TaskStatus},
    RustError,
};

use super::filter::Filter;

pub mod git;
pub mod taskchampion;

#[warn(missing_docs)]
/// This is the main trait, defining a "Repository".
/// A repository holds tasks and governs how they are synced.
/// This trait unites their interface.
pub trait StrideRepository {
    /* TODO(@bpeetz): I have no idea, what this function does <2024-10-25> */
    fn unload(&mut self);

    /// Add a [`Task`] to the Repository
    fn add(&mut self, task: Task) -> Result<(), RustError>;
    /// Remove a [`Task`], will return [`None`] if it did not exists
    fn remove_by_uuid(&mut self, uuid: &Uuid) -> Result<Option<Task>, RustError>;
    /// Remove an existing [`Task`], returning [`true`] if it was previously added
    fn remove_by_task(&mut self, task: &Task) -> Result<bool, RustError>;

    /// Try to get a [`Task`] by [`Uuid`]
    fn task_by_uuid(&mut self, uuid: &Uuid) -> Result<Option<Task>, RustError>;
    /// Get all [`Task`]s matching [`Filter`]
    fn tasks_with_filter(&mut self, filter: &Filter) -> Result<Vec<Task>, RustError>;

    /// Ensure that all previous operations are written to disk.
    fn commit(&mut self) -> Result<(), RustError>;

    fn update(&mut self, task: &Task) -> Result<bool, RustError>;

    fn change_category(&mut self, task: &Task, status: TaskStatus) -> Result<bool, RustError>;

    fn sync(&mut self) -> Result<(), RustError>;
    fn clear(&mut self) -> Result<(), RustError>;

    fn export(&mut self) -> Result<String, RustError>;
    fn import(&mut self, content: &str) -> Result<(), RustError>;
}

#[frb(ignore)]
/// An extension trait to the [`StrideRepository`] Trait.
pub trait StrideRepositoryExt: StrideRepository {
    /// Get all [`Task`]s with status [`TaskStatus::Pending`]
    fn tasks(&mut self) -> Result<Vec<Task>, RustError> {
        self.tasks_with_filter(&Filter {
            name: "default".to_owned(),
            status: HashSet::from_iter([TaskStatus::Pending]),
            uuid: Uuid::now_v7(),
            search: String::new(),
        })
    }
}
