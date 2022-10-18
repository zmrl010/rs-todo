//! # Application State
//!
//! Provides [`State`] structure for representing application state

use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::json;

const DEFAULT_LIST: &str = "[default]";

#[derive(Debug, Deserialize, Serialize)]
/// Application state structure
pub struct State {
    /// name of current active list in the index
    active_list: Option<String>,
    /// list file index using a list's name as a key to get that list's path
    list_index: HashMap<String, PathBuf>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            active_list: Some(String::from(DEFAULT_LIST)),
            list_index: HashMap::from([(
                String::from(DEFAULT_LIST),
                PathBuf::from(format!("{}.json", DEFAULT_LIST)),
            )]),
        }
    }
}

impl State {
    /// Load state from a file at `path`
    ///
    /// # Arguments
    ///
    /// * `path` - path to file being loaded from
    pub fn load<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        json::from_file(path)
    }

    /// Save state to file at `path`
    ///
    /// # Arguments
    ///
    /// * `path` - path to file being saved to
    pub fn save<P: AsRef<Path>>(&self, path: P) -> crate::Result<()> {
        json::to_file(path, &self)
    }
}

/// Update state by loading file at `path`
///
/// Set active list to the list found at `index[key]`
///
/// # Arguments
///
/// * `path` - path to file where state is persisted
/// * `key` - Key name of list in the index to make active
///
/// # Errors
///
/// If the list key doesn't exist in the index, it cannot be activated
pub fn activate_list(path: PathBuf, key: String) -> crate::Result<()> {
    let mut state = State::load(&path)?;
    if !state.list_index.contains_key(&key) {
        bail!("list `{}` doesn't exist in the index", key)
    }
    state.active_list = Some(key);

    state.save(path)
}

/// Get active list path or [`None`] if there is no active list set
pub fn get_active_path(state: &State) -> Option<&PathBuf> {
    state
        .active_list
        .as_ref()
        .and_then(|key| get_list_path(state, key))
}

/// Get a list's path from the index or [`None`] if there is no record for the `key`
pub fn get_list_path<S: AsRef<str>>(state: &State, key: S) -> Option<&PathBuf> {
    state.list_index.get(key.as_ref())
}
