//! # index module
//! Home of [`Index`], an alias of [`HashMap<String, PathBuf>`]
//! Used to store path values corresponding with name keys

use std::{collections::HashMap, path::PathBuf};

/// file index with name keys and path values
pub type Index = HashMap<String, PathBuf>;
