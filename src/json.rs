//! # json module
use std::{fs, path::Path};

use serde::{de::DeserializeOwned, Serialize};

/// Load and deserialize `T` from the file at the specified path
///
/// # Errors
///
/// Can fail while reading the file or deserializing it's data.
///
/// See: [`fs::read`] and [`serde_json::from_slice`]
pub fn from_file<P, T>(path: P) -> anyhow::Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let bytes = fs::read(&path)?;
    let value = serde_json::from_slice(&bytes)?;

    Ok(value)
}

/// Serialize `value` and save to the file at the specified path
///
/// # Arguments
///
/// * `path` - path of the file
/// * `value` - value to serialize and save
///
/// # Errors
///
/// Can fail while opening file or writing to it.
/// See: [`fs::write`] and [`serde_json::to_vec`]
pub fn to_file<P, T>(path: P, value: T) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let bytes = serde_json::to_vec(&value)?;
    fs::write(path, bytes)?;

    Ok(())
}
