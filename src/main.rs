mod cli;
mod task;

use std::path::PathBuf;

use clap::Parser;
use cli::{Action::*, CommandLineArgs};
use task::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() {
    let args = CommandLineArgs::parse();

    let file = args
        .file
        .or_else(find_default_journal_file)
        .expect("Failed to find file");

    match args.action {
        Add { text, .. } => task::add_task(file, Task::new(text)),
        List => task::list_tasks(file),
        Done { position } => task::complete_task(file, position),
    }
    .expect("Failed to perform action")
}
