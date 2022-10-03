pub mod cli;
mod task;

use std::path::PathBuf;

pub use clap::Parser;

use anyhow::anyhow;
use cli::{CommandLineArgs, TaskCommand::*};
use task::Task;

fn find_default_data_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rs-todo.json");
        path
    })
}

pub fn run(args: CommandLineArgs) -> anyhow::Result<()> {
    let file_path = args
        .file
        .or_else(find_default_data_file)
        .ok_or(anyhow!("Failed to find data file."))?;

    match args.command {
        Add { text, .. } => task::add_task(file_path, Task::new(text)),
        List => task::list_tasks(file_path),
        Done { position } => task::complete_task(file_path, position),
    }?;

    Ok(())
}
