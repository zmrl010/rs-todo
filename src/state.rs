use serde::{self, Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    /// path to active task list
    active: PathBuf,
}

impl State {
    fn new() -> Self {
        Self {
            active: PathBuf::new(),
        }
    }
}
