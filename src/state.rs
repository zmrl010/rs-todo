//! # Application State
//!
//! Provides [`State`] structure for representing application state

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

/// Application state structure
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct State {
    /// index key of current active list
    pub active_list: String,
    /// file index using name key to get path of task list
    pub index: HashMap<String, PathBuf>,
}

impl State {
    pub fn new() -> Self {
        Self {
            active_list: String::new(),
            index: HashMap::new(),
        }
    }
}
