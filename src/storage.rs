use anyhow::Context;
use std::{
    fs,
    io::{Read, Write},
    path::Path,
};

/// Base trait for implementing storage backend
pub trait StorageBackend: Read + Write {}

pub trait FileStorageBackend: StorageBackend {
    fn read<P>(path: P) -> anyhow::Result<Vec<u8>>
    where
        P: AsRef<Path>,
    {
        let data = fs::read(&path)
            .with_context(|| format!("Failed to read from {}", path.as_ref().display()))?;
        Ok(data)
    }

    fn write<P, C>(path: P, data: C) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
        C: AsRef<[u8]>,
    {
        fs::write(&path, data)
            .with_context(|| format!("Failed to write to {}", path.as_ref().display()))?;
        Ok(())
    }
}
