//! # Application State
//!
//! Provides [`State`] structure for representing application state

use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

type ListIndex = HashMap<String, PathBuf>;

#[derive(Debug, Deserialize, Serialize, Default)]
/// Application state structure
pub struct State {
    /// name of current active list in the index
    active_list: Option<String>,
    /// list file index using a list's name as a key to get that list's path
    list_index: ListIndex,
}

/// Set the list found at `index[key]` active
///
/// # Arguments
///
/// * `name` - Key of list in the index to make active
///
/// # Errors
///
/// If the list name doesn't exist in the index, it cannot be activated
pub fn activate_list(state: &mut State, key: String) -> crate::Result<()> {
    if state.list_index.contains_key(&key) {
        bail!("list `{}` doesn't exist in the index", key)
    }
    state.active_list = Some(key);
    Ok(())
}

/// Get active list path or [`None`] if there is no active list
pub fn get_active_path(state: &State) -> Option<&PathBuf> {
    state
        .active_list
        .as_ref()
        .and_then(|key| state.list_index.get(key))
}
