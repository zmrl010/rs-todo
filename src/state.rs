//! # Application State
//!
//! Provides [`State`] structure for representing application state

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

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

/// File index with name keys and path values
pub type Index = HashMap<String, PathBuf>;
