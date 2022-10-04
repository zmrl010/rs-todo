use std::{
    fmt,
    fs::File,
    io::{self, Error, ErrorKind, Seek, SeekFrom},
    path::PathBuf,
};

use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

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

fn collect_tasks(mut file: &File) -> io::Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; // Rewind before
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?; // Rewind before

    Ok(tasks)
}

pub fn add_task(path: PathBuf, task: Task) -> io::Result<()> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let mut tasks = collect_tasks(&file)?;

    tasks.push(task);

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(path: PathBuf, task_position: usize) -> io::Result<()> {
    let file = File::options().read(true).write(true).open(path)?;
    let mut tasks = collect_tasks(&file)?;
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks[task_position - 1].complete = true;
    file.set_len(0)?;

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn list_tasks(path: PathBuf) -> io::Result<()> {
    let file = File::open(path)?;
    let tasks = collect_tasks(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        for (i, task) in tasks.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }

    Ok(())
}
