mod cli;
mod json;
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
use state::State;
use tasks::Task;

const DEFAULT_DATA_DIR: &str = ".rs-todo/";

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
        find_data_dir(DEFAULT_DATA_DIR).ok_or_else(|| anyhow!("Failed to find data directory"))?;

    fs::create_dir_all(&data_dir)?;

    let state_path = data_dir.with_file_name(".state.json");
    let State {
        active_list,
        mut index,
    } = json::from_file(state_path).or_else(|_| anyhow::Ok(State::new()))?;

    let list_path = index
        .entry(active_list)
        .or_insert(data_dir.with_file_name("[default].json"));

    match args.command {
        Add { text } => tasks::add_task(list_path, Task::new(text)),
        List => tasks::list_all(list_path),
        Done { position } => tasks::complete_task(list_path, position),
    }?;

    Ok(())
}
