//! # task module
//!
//! Provides [`Task`] structure representing a single task  
//! and related operations

use anyhow::ensure;
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{self, Deserialize, Serialize};
use std::{fmt, path::PathBuf};

use crate::{fs, json, task_list::TaskList};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    text: String,
    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>,
    complete: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

impl Task {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            text: text.as_ref().to_string(),
            created_at: Utc::now(),
            complete: false,
        }
    }
}

/// [`Task`] factory - initialize a [`Task`] by supplying text
///
/// # Examples
///
/// ```rust
/// use task;
///
/// fn main() {
///     let t = task::create("Brush teeth");
///     assert_eq!(t.text, "Brush teeth");
/// }
///
/// ```
pub fn create<S: AsRef<str>>(text: S) -> Task {
    Task::new(text)
}

pub fn add_task(path: PathBuf, task: Task) -> anyhow::Result<()> {
    let file = fs::open_file(path)?;

    let task_list = {
        let mut task_list: TaskList = json::read(&file)?;
        task_list.push(task);
        task_list
    };

    json::write(file, task_list)
}

pub fn complete_task(path: PathBuf, position: usize) -> anyhow::Result<()> {
    let file = fs::open_file(path)?;

    fn set_task_complete(task: &mut Task) -> Option<Task> {
        task.complete = true;
        None
    }

    let task_list = {
        let mut task_list: TaskList = json::read(&file)?;

        ensure!(
            position > 0 && position <= task_list.len(),
            "Invalid `position` (expected 0 < *n* <= {}, found {})",
            task_list.len(),
            position,
        );

        task_list.get_mut(position - 1).and_then(set_task_complete);
        task_list
    };

    json::write(file, task_list)
}

pub fn list_tasks(path: PathBuf) -> anyhow::Result<()> {
    let task_list: TaskList = json::read_file(path)?;

    println!("{}", task_list);

    Ok(())
}
