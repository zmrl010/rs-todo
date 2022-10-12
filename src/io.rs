//! # I/O Operations

use anyhow::Context;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

/// Read bytes from a reader
pub fn read<R: Read>(rdr: R) -> anyhow::Result<Vec<u8>> {
    let mut buf_reader = BufReader::new(rdr);
    let mut buffer = Vec::new();
    buf_reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Open a file with options `read`, `write` and `create`
///
/// The file will be created if it doesn't exist,
/// otherwise it will be overwritten when data is wrote to it
pub fn open_file<P: AsRef<Path>>(path: P) -> anyhow::Result<File> {
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
pub fn read_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<u8>> {
    let file = open_file(path)?;
    let bytes = read(file)?;
    Ok(bytes)
}
