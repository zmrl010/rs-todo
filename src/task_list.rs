//! # task_list module
//!
//! Provides [`TaskList`] structure representing a collection of tasks
//! and related operations

use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use crate::task::Task;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TaskList(Vec<Task>);

impl TaskList {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

/// Implements string formatting for TaskList
impl fmt::Display for TaskList {
    /// # Overflow Behavior
    ///
    /// This method uses [`Iterator::enumerate`] to generate indexes as [`usize`], which does not
    /// guard against overflows, so enumerating more than [`usize::MAX`] elements either
    /// produces the wrong result or panics. If debug assertions are enabled, a panic is guaranteed.
    ///
    /// # Panics
    ///
    /// The internal iterator might panic if the to-be-returned index would
    /// overflow a [`usize`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "Task list is empty!")?;
        } else {
            for (i, task) in self.iter().enumerate() {
                writeln!(f, "{}: {}", i + 1, task)?;
            }
        }
        Ok(())
    }
}

impl Deref for TaskList {
    type Target = Vec<Task>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TaskList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
