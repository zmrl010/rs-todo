mod cli;
mod task;

use std::path::PathBuf;

use anyhow::anyhow;
use clap::Parser;
use cli::{Action::*, CommandLineArgs};
use task::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let args = CommandLineArgs::parse();

    let file = args
        .file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    match args.action {
        Add { text, .. } => task::add_task(file, Task::new(text)),
        List => task::list_tasks(file),
        Done { position } => task::complete_task(file, position),
    }?;
    Ok(())
}
