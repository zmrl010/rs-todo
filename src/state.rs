use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use serde::{self, Deserialize, Serialize};

/// Tuple representing a record composed of a title and path
#[derive(Debug, Deserialize, Serialize)]
pub struct NamePath(String, PathBuf);

impl NamePath {
    fn new() -> Self {
        NamePath(String::new(), PathBuf::new())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    /// row index of selected list
    active: usize,
    lists: Vec<NamePath>,
}

impl State {
    fn new() -> Self {
        State {
            active: 0,
            lists: Vec::new(),
        }
    }

    fn activate(&mut self, index: usize) {
        self.active = index
    }
}

pub fn from_reader(mut rdr: impl Read) -> anyhow::Result<State> {
    let mut buffer = String::new();
    rdr.read_to_string(&mut buffer)?;

    let index: State = serde_json::from_str(buffer.as_str())?;

    Ok(index)
}

pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<State> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    from_reader(file).or_else(|_| Ok(State::new()))
}
