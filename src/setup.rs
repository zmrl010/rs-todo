use dirs;
use std::path::PathBuf;

pub fn find_default_data_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|mut path| {
        path.push(".rs-todo/");
        path
    })
}

// Create and setup application data directory if one does not exist
// pub fn initialize_data_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {}
