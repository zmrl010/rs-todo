mod cli;
mod io;
mod state;
mod task;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
pub use anyhow::Result;
use cli::TaskCommand::*;
pub use cli::{parse, CommandLineArgs};
use task::Task;

/// Get application data directory starting from system user's data directory
///
/// Uses [`dirs::data_dir`]
fn find_data_dir<P: AsRef<Path>>(dir: P) -> Option<PathBuf> {
    fn inner(dir: &Path) -> Option<PathBuf> {
        dirs::data_dir().map(|mut path| {
            path.push(dir);
            path
        })
    }

    inner(dir.as_ref())
}

/// Start application
pub fn run(args: CommandLineArgs) -> anyhow::Result<()> {
    let data_dir =
        find_data_dir(".rs-todo/").ok_or_else(|| anyhow!("Failed to find data directory"))?;

    fs::create_dir_all(&data_dir)?;

    match args.command {
        Add { text } => task::add_task(data_dir, Task::new(text)),
        List => task::list_all(data_dir),
        Done { position } => task::complete_task(data_dir, position),
    }?;

    Ok(())
}
