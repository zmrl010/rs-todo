mod cli;
mod json;
mod state;
mod tasks;

use std::{fs, path::PathBuf};

use anyhow::{anyhow, Context};
use cli::TaskCommand::{self, *};
use state::State;
use tasks::Task;

pub use anyhow::Result;
pub use cli::{parse, CommandLineArgs};

/// Get application data directory starting from system user's data directory
///
/// [`dirs::data_dir`]
fn find_default_data_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|mut data_dir| {
        data_dir.push(".rs-todo/");
        data_dir
    })
}

fn run_task_command(command: TaskCommand, path: &PathBuf) -> crate::Result<()> {
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
        .with_context(|| format!("failed `fs::create_dir_all({})`", &data_dir.display()))?;

    let state_file_path = data_dir.with_file_name(".state.json");
    let state = State::load(&state_file_path)?;

    let default_list_file = data_dir.with_file_name("[default].json");
    let active_list_path = state.get_active_path().unwrap_or(&default_list_file);

    run_task_command(args.command, active_list_path)?;

    state.save(&state_file_path)?;

    Ok(())
}
