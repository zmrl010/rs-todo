//! # Filesystem Extensions
//!
//! Extensions and helpers for working with [`std::fs`]

use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

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
        .with_context(|| {
            format!(
                "Failed filesystem operation at: \"{}\"",
                path.as_ref().display()
            )
        })?;
    Ok(file)
}

/// Read a file from `path`
///
/// Similar to [`std::fs::read`], but uses [`crate::fsx::open_file`] which
/// will create a file if it doesn't exist
pub fn read_file<P>(path: P) -> anyhow::Result<Vec<u8>>
where
    P: AsRef<Path>,
{
    fn inner(path: &Path) -> anyhow::Result<Vec<u8>> {
        let file = open_file(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut buffer = Vec::new();
        buf_reader.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
    inner(path.as_ref())
}
