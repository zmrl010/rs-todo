//! # tasks module
//!
//! Provides [`Task`] structure representing a single task  
//! and related operations

use anyhow::bail;
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::File,
    io::{BufReader, Read, Seek},
    path::PathBuf,
};

use crate::json;

#[derive(Debug, Deserialize, Serialize)]
pub enum ErrorKind {}

/// Structure to represent a single task
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
    pub fn new(text: String) -> Self {
        Self {
            text,
            created_at: Utc::now(),
            complete: false,
        }
    }
}

/// Read bytes from a reader
fn read_all_bytes<R: Read>(rdr: R) -> crate::Result<Vec<u8>> {
    let mut buf_reader = BufReader::new(rdr);
    let mut buffer = Vec::new();
    buf_reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Read serialized task list from a file into a [`Vec<Task>`]
fn collect_tasks(mut file: &File) -> crate::Result<Vec<Task>> {
    file.rewind()?; // Rewind before
    let bytes = read_all_bytes(file)?;
    let tasks = json::from_slice(&bytes).or_else(|err| {
        if err.is_eof() {
            return Ok(Vec::new());
        }
        Err(err)
    })?;
    file.rewind()?; // Rewind after

    Ok(tasks)
}

/// Append [`Task`] to a list
///
/// # Arguments
///
/// * `path` - location of the list
/// * `task` - task to add
pub fn add_task(path: &PathBuf, task: Task) -> crate::Result<()> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)?;

    let task_list = {
        let mut task_list = collect_tasks(&file)?;
        task_list.push(task);
        task_list
    };

    json::to_writer(file, &task_list)?;
    Ok(())
}

/// Mark [`Task`] as completed in a list at the file `path`
///
/// # Arguments
///
/// * `path` - location of the list
/// * `position` - item's index in the list **1-based**
pub fn complete_task(path: &PathBuf, position: usize) -> crate::Result<()> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)?;

    let task_list = {
        let mut task_list = collect_tasks(&file)?;
        if position == 0 || position > task_list.len() {
            bail!(
                "Invalid `position` (expected 0 < *n* <= {}, found {})",
                task_list.len(),
                position,
            )
        }
        if let Some(mut task) = task_list.get_mut(position - 1) {
            task.complete = true;
        }
        task_list
    };

    json::to_writer(file, &task_list)?;

    Ok(())
}

/// List all [`Task`]s in a list
///
/// # Arguments
///
/// * `path` - location of the list
pub fn list_all(path: &PathBuf) -> crate::Result<()> {
    let file = File::options().read(true).open(&path)?;
    let task_list = collect_tasks(&file)?;

    if task_list.is_empty() {
        println!("Task list is empty!");
    } else {
        for (i, task) in task_list.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }

    Ok(())
}
