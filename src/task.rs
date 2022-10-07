use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::File,
    io::{self, BufReader, Error, ErrorKind, Read, Seek},
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

fn collect_tasks(mut file: &File) -> io::Result<Vec<Task>> {
    file.rewind()?; // Rewind before
    let mut contents = Vec::new();
    let mut buf = BufReader::new(file);
    buf.read_to_end(&mut contents)?;
    // serde_json::from_slice(&bytes)?;
    let tasks = match serde_json::from_slice(&contents) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    file.rewind()?; // Rewind after

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

    tasks.get_mut(task_position - 1).and_then(|task| {
        task.complete = true;
        Some(())
    });

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn list_tasks(path: PathBuf) -> io::Result<()> {
    let file = File::options().read(true).open(path)?;
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
