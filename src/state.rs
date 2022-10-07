use serde::{self, Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

/// file index with name keys and path values
pub type Index = HashMap<String, PathBuf>;

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    /// path to active list
    active: PathBuf,
}

impl State {
    fn new() -> Self {
        State {
            active: PathBuf::new(),
        }
    }
}
