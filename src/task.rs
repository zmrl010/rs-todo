use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fmt, fs,
    io::{self, Error, ErrorKind},
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
    pub fn new(text: String) -> Task {
        Task {
            text,
            created_at: Utc::now(),
            complete: false,
        }
    }
}

fn collect_tasks(path: &PathBuf) -> io::Result<Vec<Task>> {
    let contents = fs::read(path)?;
    let tasks = serde_json::from_slice(&contents)?;
    Ok(tasks)
}

fn write_tasks(path: &PathBuf, tasks: Vec<Task>) -> io::Result<()> {
    let contents = serde_json::to_vec(&tasks)?;
    fs::write(path, contents)
}

pub fn add_task(path: PathBuf, task: Task) -> io::Result<()> {
    let mut tasks = collect_tasks(&path)?;
    tasks.push(task);
    write_tasks(&path, tasks)
}

pub fn complete_task(path: PathBuf, position: usize) -> io::Result<()> {
    let mut tasks = collect_tasks(&path)?;
    if position == 0 || position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }

    tasks.get_mut(position - 1).and_then(|task| {
        task.complete = true;
        Some(())
    });

    write_tasks(&path, tasks)
}

pub fn list_tasks(path: PathBuf) -> io::Result<()> {
    let tasks = collect_tasks(&path)?;
    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        for (i, task) in tasks.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }

    Ok(())
}
