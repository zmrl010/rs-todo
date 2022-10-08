pub mod cli;
mod state;
mod storage;
mod task;

use anyhow::anyhow;
pub use anyhow::Result;
pub use clap::Parser;
pub use cli::CommandLineArgs;
use cli::TaskCommand::*;
use dirs;
use std::path::PathBuf;
use task::Task;

fn find_default_data_file() -> Option<PathBuf> {
    dirs::data_dir().map(|mut path| {
        path.push(".rs-todo.json");
        path
    })
}

/// Parse from `std::env::args_os()`, exit on error
///
/// Convenience function for [clap::derive::Parser::parse]
pub fn parse() -> CommandLineArgs {
    CommandLineArgs::parse()
}

/// Start application
pub fn run(args: CommandLineArgs) -> anyhow::Result<()> {
    let file_path = args
        .file
        .or_else(find_default_data_file)
        .ok_or(anyhow!("Failed to find data file."))?;

    match args.command {
        Add { text } => task::add_task(file_path, Task::new(text)),
        List => task::list_tasks(file_path),
        Done { position } => task::complete_task(file_path, position),
    }?;

    Ok(())
}
