//! # task module
//!
//! Provides [`Task`] structure representing a single task  
//! and related operations

use anyhow::bail;
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub complete: bool,
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

pub type TaskList = Vec<Task>;

/// Append [`Task`] to a list
///
/// * `path` - location of the list
/// * `task` - task to add
pub fn add_task(path: PathBuf, task: Task) -> anyhow::Result<()> {
    let file = crate::io::open_file(path)?;
    let bytes = crate::io::read(&file)?;
    let task_list = {
        let mut task_list: TaskList = serde_json::from_slice(&bytes)?;
        task_list.push(task);
        task_list
    };

    serde_json::to_writer(file, &task_list)?;
    Ok(())
}

/// Mark [`Task`] as completed in a list at the file `path`
///
/// * `path` - location of the list
/// * `position` - item's index in the list **1-based**
pub fn complete_task(path: PathBuf, position: usize) -> anyhow::Result<()> {
    let file = crate::io::open_file(path)?;
    let bytes = crate::io::read(&file)?;
    let task_list: TaskList = serde_json::from_slice(&bytes)?;
    if position == 0 || position > task_list.len() {
        bail!(
            "Invalid `position` (expected 0 < *n* <= {}, found {})",
            task_list.len(),
            position,
        )
    }

    let task_list = {
        let mut task_list = task_list;
        if let Some(mut task) = task_list.get_mut(position - 1) {
            task.complete = true;
        }
        task_list
    };

    serde_json::to_writer(file, &task_list)?;

    Ok(())
}

/// List all [`Task`]s in a list
///
/// * `path` - location of the list
pub fn list_all(path: PathBuf) -> anyhow::Result<()> {
    let bytes = crate::io::read_file(path)?;
    let task_list: TaskList = serde_json::from_slice(&bytes)?;

    if task_list.is_empty() {
        println!("Task list is empty!");
    } else {
        for (i, task) in task_list.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }

    Ok(())
}
