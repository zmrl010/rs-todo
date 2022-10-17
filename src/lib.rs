mod cli;
mod json;
mod state;
mod tasks;

use std::{fs, path::PathBuf};

use anyhow::{anyhow, Context};
use cli::Commands::{self, *};
use state::State;
use tasks::Task;

pub use anyhow::Result;
pub use cli::{parse, CommandLineArgs};

const DATA_DIRECTORY: &str = ".rs-todo/";
const STATE_FILE_NAME: &str = ".state.json";
const DEFAULT_LIST_FILE_NAME: &str = "[default].json";

/// Get application data directory starting from system user's data directory
///
/// [`dirs::data_dir`]
fn find_default_data_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|mut data_dir| {
        data_dir.push(DATA_DIRECTORY);
        data_dir
    })
}

/// delegate command from [`Commands`]
fn run_command(command: Commands, path: &PathBuf) -> crate::Result<()> {
    match command {
        Add { text } => tasks::add_task(path, Task::new(text)),
        List => tasks::list_all(path),
        Done { position } => tasks::complete_task(path, position),
    }?;
    Ok(())
}

/// Start application
pub fn run(args: CommandLineArgs) -> crate::Result<()> {
    let data_dir = args
        .data_dir
        .or_else(find_default_data_dir)
        .ok_or_else(|| anyhow!("failed to find data directory"))?;
    // create parent dirs if they don't already exist
    fs::create_dir_all(&data_dir)
        .with_context(|| format!("failed to create `{}`", &data_dir.display()))?;

    let state_file_path = data_dir.with_file_name(STATE_FILE_NAME);
    let app_state: State = json::from_file(&state_file_path)?;

    let default_list_path = data_dir.with_file_name(DEFAULT_LIST_FILE_NAME);
    let active_list_path = state::get_active_path(&app_state).unwrap_or(&default_list_path);

    run_command(args.command, active_list_path)?;

    // persist any state changes back to storage
    json::to_file(&state_file_path, app_state)?;

    Ok(())
}
