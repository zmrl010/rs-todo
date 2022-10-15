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

const DEFAULT_FILE_NAME: &str = "[default]";

#[derive(Debug, Deserialize, Serialize)]
/// Application state structure
pub struct State {
    /// name of current active list in the index
    active_list: String,
    /// list file index using the key of a list's name to get that list's path
    list_index: Index,
}

impl Default for State {
    fn default() -> Self {
        Self {
            active_list: String::from(DEFAULT_FILE_NAME),
            list_index: Index::default(),
        }
    }
}

impl State {
    /// Load state from a file at `path`
    ///
    /// # Arguments
    ///
    /// * `path` - path to file being loaded from
    ///
    pub fn load<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        json::from_file(path)
    }

    /// Save state to file at `path`
    ///
    /// # Arguments
    ///
    /// * `path` - path to file being saved to
    ///
    pub fn save<P: AsRef<Path>>(&self, path: P) -> crate::Result<()> {
        json::to_file(path, &self)
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
    pub fn activate<S: AsRef<str>>(&mut self, name: &S) -> crate::Result<()> {
        let name = name.as_ref().to_string();
        if !self.list_index.contains_name(&name) {
            bail!("name `{}` doesn't exist in the index", name)
        }
        self.active_list = name;
        Ok(())
    }

    /// Get active list path or [`None`] if there is no active list
    pub fn get_active_path(&self) -> Option<&PathBuf> {
        self.list_index.get(&self.active_list)
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// List file index with named keys that point to the corresponding list's filepath
///
/// See [`HashMap<String, PathBuf>`]
pub struct Index(HashMap<String, PathBuf>);

impl Index {
    /// Return reference to underlying [`HashMap`]
    pub fn get_map(&self) -> &HashMap<String, PathBuf> {
        &self.0
    }

    /// Inserts a name-path pair into the index
    pub fn insert(&mut self, name: String, path: PathBuf) -> Option<PathBuf> {
        self.0.insert(name, path)
    }

    /// Returns a reference to the value corresponding to the name
    pub fn get(&self, name: &String) -> Option<&PathBuf> {
        self.0.get(name)
    }

    /// Returns `true` if the index contains a list by the specified name
    pub fn contains_name(&self, name: &String) -> bool {
        self.0.contains_key(name)
    }
}

impl Default for Index {
    fn default() -> Self {
        Self(HashMap::from([(
            String::from(DEFAULT_FILE_NAME),
            PathBuf::from(format!("{}.json", DEFAULT_FILE_NAME)),
        )]))
    }
}
