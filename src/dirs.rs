//! Application-specific directory getters
//!
//! Expands upon [`dirs`]

use std::path::PathBuf;

const APP_DATA_DIRECTORY: &str = ".rs-todo/";

/// Get application data directory starting from system user's data directory
///
/// See [`dirs::data_dir`]
pub fn data_dir() -> Option<PathBuf> {
    fn push_dir(mut data_dir: PathBuf) -> PathBuf {
        data_dir.push(APP_DATA_DIRECTORY);
        data_dir
    }

    dirs::data_dir().map(push_dir)
}
