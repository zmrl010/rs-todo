mod cli;
mod json;
mod state;
mod tasks;

use std::{fs, path::PathBuf};

use anyhow::anyhow;
pub use anyhow::Result;
use cli::TaskCommand::*;
pub use cli::{parse, CommandLineArgs};
use state::State;
use tasks::Task;

/// Get application data directory starting from system user's data directory
///
/// [`dirs::data_dir`]
fn find_default_data_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|mut data_dir| {
        data_dir.push(".rs-todo/");
        data_dir
    })
}

/// Start application
pub fn run(args: CommandLineArgs) -> anyhow::Result<()> {
    let data_dir = args
        .data_dir
        .or_else(find_default_data_dir)
        .ok_or_else(|| anyhow!("Failed to find data directory"))?;

    fs::create_dir_all(&data_dir)?;

    let state_path = data_dir.with_file_name(".state.json");
    let State {
        active_list,
        mut index,
    } = json::from_file(state_path).or_else(|_| anyhow::Ok(State::default()))?;

    let list_path = index
        .entry(active_list)
        .or_insert_with_key(|key| data_dir.with_file_name(format!("{}.json", key)));

    match args.command {
        Add { text } => tasks::add_task(list_path, Task::new(text)),
        List => tasks::list_all(list_path),
        Done { position } => tasks::complete_task(list_path, position),
    }?;

    Ok(())
}
