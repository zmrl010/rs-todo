mod cli;
mod fs;
mod index;
mod json;
mod setup;
mod state;
mod task;
mod task_list;

use anyhow::anyhow;
pub use anyhow::Result;
use cli::TaskCommand::*;
pub use cli::{parse, CommandLineArgs};

/// Start application
pub fn run(args: CommandLineArgs) -> anyhow::Result<()> {
    let file_path = args
        .file
        .or_else(setup::find_default_data_dir)
        .ok_or(anyhow!("Failed to find data file."))?;

    match args.command {
        Add { text } => task::add_task(file_path, task::create(text)),
        List => task::list_tasks(file_path),
        Done { position } => task::complete_task(file_path, position),
    }?;

    Ok(())
}
