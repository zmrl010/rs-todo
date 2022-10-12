//! # JSON module
//!
//! Handles interop between the filesystem and
//! JSON serialization / deserialization

use std::io::{self, BufReader, Read};

use serde::de::DeserializeOwned;
pub use serde_json::*;

/// Read `deserialize`-able JSON into `T`
///
/// `T` implements [`Deserialize`]
///
/// [`Read::read`] -> [`Deserialize::deserialize`] -> `T`
pub fn read<R, T>(rdr: R) -> anyhow::Result<T>
where
    R: Read,
    T: DeserializeOwned,
{
    fn inner_read<R: Read>(rdr: R) -> io::Result<Vec<u8>> {
        let mut buf_reader = BufReader::new(rdr);
        let mut buffer = Vec::new();
        buf_reader.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
    let contents = inner_read(rdr)?;
    let value = from_slice(&contents)?;
    Ok(value)
}
