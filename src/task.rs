use anyhow::anyhow;
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{self, Deserialize, Serialize};
use std::{
    fmt,
    fs::File,
    io::{self, BufReader, Read, Write},
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

/// Retrieve tasks from storage
fn collect_tasks<R: Read>(rdr: R) -> anyhow::Result<TaskList> {
    let contents = {
        let mut buf_reader = BufReader::new(rdr);
        let mut contents = Vec::new();
        buf_reader.read_to_end(&mut contents)?;
        contents
    };

    let tasks = serde_json::from_slice(&contents)?;
    Ok(tasks)
}

/// Save tasks to storage, overwriting any existing data if it exists
fn commit_tasks<W: Write>(writer: W, task_list: TaskList) -> anyhow::Result<()> {
    serde_json::to_writer(writer, &task_list)?;
    Ok(())
}

pub fn add_task(path: PathBuf, task: Task) -> anyhow::Result<()> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let task_list = {
        let mut task_list = collect_tasks(&file)?;
        task_list.0.push(task);
        task_list
    };

    commit_tasks(&file, task_list)
}

pub fn complete_task(path: PathBuf, position: usize) -> anyhow::Result<()> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let mut task_list = collect_tasks(&file)?;
    if position == 0 || position > task_list.0.len() {
        let err = io::Error::new(io::ErrorKind::InvalidInput, "Invalid Task ID");
        return Err(anyhow!(err));
    }

    fn set_task_complete(task: &mut Task) -> Option<Task> {
        task.complete = true;
        None
    }

    task_list
        .0
        .get_mut(position - 1)
        .and_then(set_task_complete);

    commit_tasks(&file, task_list)
}

pub fn list_tasks(path: PathBuf) -> anyhow::Result<()> {
    let file = File::options().read(true).open(path)?;
    let task_list = collect_tasks(&file)?;
    if task_list.0.is_empty() {
        println!("Task list is empty!");
    } else {
        for (i, task) in task_list.0.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }

    Ok(())
}
