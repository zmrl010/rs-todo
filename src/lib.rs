mod cli;
mod state;
mod tasks;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
pub use anyhow::Result;
use cli::TaskCommand::*;
pub use cli::{parse, CommandLineArgs};
use tasks::Task;

/// Get application data directory starting from system user's data directory
///
/// [`dirs::data_dir`]
fn find_data_dir<P: AsRef<Path>>(dir: P) -> Option<PathBuf> {
    dirs::data_dir().map(|mut data_dir| {
        data_dir.push(dir);
        data_dir
    })
}

/// Start application
pub fn run(args: CommandLineArgs) -> anyhow::Result<()> {
    let data_dir =
        find_data_dir(".rs-todo/").ok_or_else(|| anyhow!("Failed to find data directory"))?;

    fs::create_dir_all(&data_dir)?;

    match args.command {
        Add { text } => tasks::add_task(data_dir, Task::new(text)),
        List => tasks::list_all(data_dir),
        Done { position } => tasks::complete_task(data_dir, position),
    }?;

    Ok(())
}
