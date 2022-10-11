//! # Filesystem operations
//!
//! extends [`std::fs`]

use std::{fs::File, path::Path};

use anyhow::Context;

/// Open a file with options `read`, `write` and `create`
///
/// The file will be created if it doesn't exist,
/// otherwise it will be overwritten when data is wrote to it
///
/// See: [`std::fs::OpenOptions`]
pub fn open_file<P>(path: P) -> anyhow::Result<File>
where
    P: AsRef<Path>,
{
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .with_context(|| format!("Failed to open file at \"{}\"", path.as_ref().display()))?;
    Ok(file)
}
