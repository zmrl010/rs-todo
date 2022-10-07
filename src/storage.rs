pub mod json {
    use serde::{self, de::DeserializeOwned, Serialize};
    use std::{fs, path::Path};

    /// Composes [fs::read] and [serde_json::from_slice]
    /// to read JSON data from a file at `path`
    pub fn read_from_file<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> anyhow::Result<T> {
        let bytes = fs::read(path)?;
        let data: T = serde_json::from_slice(&bytes)?;
        Ok(data)
    }

    /// Composes [fs::write] and [serde_json::to_string]
    /// to write JSON data to a file at `path`
    pub fn write_to_file<P: AsRef<Path>, T: ?Sized + Serialize>(
        path: P,
        data: &T,
    ) -> anyhow::Result<()> {
        let contents = serde_json::to_string(data)?;
        fs::write(path, contents)?;
        Ok(())
    }
}
