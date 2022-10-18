mod cli;
mod dirs;
mod json;
mod state;
mod tasks;

use std::{fs, path::PathBuf};

use anyhow::{anyhow, Context};
use cli::Commands::*;
use state::State;
use tasks::Task;

pub use anyhow::Result;
pub use cli::{parse, CommandLineArgs};

const STATE_FILE_NAME: &str = ".state.json";

/// Create directory and any parents if they don't already exist
///
/// See [`fs::create_dir_all`] and [`anyhow::Context`]
fn create_dir_all(path: &PathBuf) -> crate::Result<()> {
    fs::create_dir_all(path).with_context(|| format!("failed to create `{}`", path.display()))
}

/// Find a default data directory if the passed `data_dir` [`Option`] is [`None`].
/// Create any components in the path that don't exist
fn resolve_data_dir(data_dir: Option<PathBuf>) -> crate::Result<PathBuf> {
    data_dir
        .or_else(dirs::data_dir)
        .ok_or_else(|| anyhow!("failed to find data directory"))
        .and_then(|dir| {
            create_dir_all(&dir)?;
            Ok(dir)
        })
}

/// Start application
pub fn run(args: CommandLineArgs) -> crate::Result<()> {
    let data_dir = resolve_data_dir(args.data_dir)?;

    let state_file_path = data_dir.with_file_name(STATE_FILE_NAME);
    let app_state: State = json::from_file(&state_file_path)?;

    let active_list_path = args
        .list
        .and_then(|key| state::get_list_path(&app_state, &key))
        .or_else(|| state::get_active_path(&app_state))
        .ok_or_else(|| anyhow!("No active path"))?;

    match args.command {
        Activate { key } => state::activate_list(state_file_path, key),
        Add { text } => tasks::add_task(active_list_path, Task::new(text)),
        List => tasks::list_all(active_list_path),
        Done { position } => tasks::complete_task(active_list_path, position),
    }?;

    Ok(())
}
