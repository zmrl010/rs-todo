//! # Application State
//!
//! Provides [`State`] structure for representing application state

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// Application state structure
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct State {
    /// index key of current active list
    pub active_list: String,
    /// file index using name key to get path of task list
    pub index: HashMap<String, PathBuf>,
}
