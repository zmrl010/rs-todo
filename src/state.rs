use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use serde::{self, Deserialize, Serialize};

/// Tuple representing a record composed of a title and path
#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    name: String,
    path: PathBuf,
}

impl Record {
    fn new() -> Self {
        Record {
            name: String::new(),
            path: PathBuf::new(),
        }
    }
}

pub type Index = Vec<Record>;

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    /// path to active list
    active: PathBuf,
    /// path to list index
    list_index: PathBuf,
}

impl State {
    fn new() -> Self {
        State {
            active: PathBuf::new(),
            list_index: PathBuf::new(),
        }
    }
}

pub fn from_reader(mut rdr: impl Read) -> anyhow::Result<State> {
    let mut buffer = String::new();
    rdr.read_to_string(&mut buffer)?;

    serde_json::from_str(buffer.as_str()).or_else(|_| Ok(State::new()))
}

pub fn read_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<State> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    from_reader(file)
}

pub fn write_to_file<P: AsRef<Path>>(path: P, state: State) -> anyhow::Result<()> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    serde_json::to_writer(file, &state)?;

    Ok(())
}
