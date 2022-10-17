//! # Application State
//!
//! Provides [`State`] structure for representing application state

use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Deserialize, Serialize, Default)]
/// Application state structure
pub struct State {
    /// name of current active list in the index
    active_list: Option<String>,
    /// list file index using a list's name as a key to get that list's path
    list_index: HashMap<String, PathBuf>,
}

impl State {
    /// Set the list found at `index[key]` active
    ///
    /// # Arguments
    ///
    /// * `name` - Key of list in the index to make active
    ///
    /// # Errors
    ///
    /// If the list name doesn't exist in the index, it cannot be activated
    pub fn activate<S: AsRef<str>>(&mut self, name: &S) -> crate::Result<()> {
        let name = name.as_ref().to_string();
        if self.list_index.contains_key(&name) {
            bail!("name `{}` doesn't exist in the index", name)
        }
        self.active_list = Some(name);
        Ok(())
    }

    /// Get active list path or [`None`] if there is no active list
    pub fn get_active_path(&self) -> Option<&PathBuf> {
        if let Some(key) = &self.active_list {
            return self.list_index.get(key);
        }
        None
    }
}
