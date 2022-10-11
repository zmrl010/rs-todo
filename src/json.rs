//! # json module
//!
//! Handles interop between the filesystem and
//! JSON serialization / deserialization

use std::{
    io::{self, BufReader, Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::fs;

/// Read `deserialize`-able JSON into `T`
///
/// `T` implements [`Deserialize`]
///
/// [`Read::read`] -> [`Deserialize::deserialize`] -> `T`
pub fn read<R, T>(rdr: R) -> anyhow::Result<T>
where
    R: Read,
    T: for<'a> Deserialize<'a>,
{
    fn inner_read<R: Read>(rdr: R) -> io::Result<Vec<u8>> {
        let mut buf_reader = BufReader::new(rdr);
        let mut buffer = Vec::new();
        buf_reader.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
    let contents = inner_read(rdr)?;
    let value = serde_json::from_slice(&contents)?;
    Ok(value)
}

pub fn read_file<P, T>(path: P) -> anyhow::Result<T>
where
    P: AsRef<Path>,
    T: for<'a> Deserialize<'a>,
{
    let file = fs::open_file(path)?;
    read(file)
}

/// Serialize `T` into JSON and write to writer, overwriting any existing data
///
/// `T` -> [`Serialize::serialize`] -> [`Write::write`]
pub fn write<W, T>(writer: W, value: T) -> anyhow::Result<()>
where
    W: Write,
    T: Serialize,
{
    serde_json::to_writer(writer, &value)?;
    Ok(())
}

pub fn write_file<P, T>(path: P, value: T) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let file = fs::open_file(path)?;
    write(file, value)
}
