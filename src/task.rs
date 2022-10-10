use anyhow::{ensure, Context};
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{self, Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::{
    fmt,
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
};

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
    pub fn new(text: String) -> Self {
        Self {
            text,
            created_at: Utc::now(),
            complete: false,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct TaskList(Vec<Task>);

impl TaskList {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

/// Implements string formatting for TaskList
impl fmt::Display for TaskList {
    /// # Overflow Behavior
    ///
    /// This method uses [`Enumerate`] to generate indexes as [`usize`], which does not
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

/// Retrieve tasks from storage
fn collect_tasks<R: Read>(rdr: R) -> anyhow::Result<TaskList> {
    let contents = {
        let mut buf_reader = BufReader::new(rdr);
        let mut buffer = Vec::new();
        buf_reader.read_to_end(&mut buffer)?;
        buffer
    };

    let tasks = serde_json::from_slice(&contents)?;
    Ok(tasks)
}

/// Save tasks to storage, overwriting any existing data if it exists
fn commit_tasks<W: Write>(writer: W, value: TaskList) -> anyhow::Result<()> {
    serde_json::to_writer(writer, &value)?;
    Ok(())
}

pub fn add_task(path: PathBuf, task: Task) -> anyhow::Result<()> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .with_context(|| format!("Failed to open file \"{}\"", path.display()))?;

    let task_list = {
        let mut task_list = collect_tasks(&file)?;
        task_list.push(task);
        task_list
    };

    commit_tasks(file, task_list)
}

pub fn complete_task(path: PathBuf, position: usize) -> anyhow::Result<()> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .with_context(|| format!("Failed to open file \"{}\"", path.display()))?;

    fn set_task_complete(task: &mut Task) -> Option<Task> {
        task.complete = true;
        None
    }

    let task_list = {
        let mut task_list = collect_tasks(&file)?;

        ensure!(
            position > 0 && position <= task_list.len(),
            "Invalid `position` (expected 0 < *n* <= {}, found {})",
            task_list.len(),
            position,
        );

        task_list.get_mut(position - 1).and_then(set_task_complete);
        task_list
    };

    commit_tasks(&file, task_list)
}

pub fn list_tasks(path: PathBuf) -> anyhow::Result<()> {
    let file = File::options()
        .read(true)
        .open(&path)
        .with_context(|| format!("Failed to open file \"{}\"", path.display()))?;
    let task_list = collect_tasks(&file)?;

    println!("{}", task_list);

    Ok(())
}
