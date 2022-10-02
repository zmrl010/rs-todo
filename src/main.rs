mod cli;
mod task;

use clap::Parser;
use cli::{Action::*, CommandLineArgs};
use task::Task;

fn main() {
    let args = CommandLineArgs::parse();

    let file = args.file.expect("Failed to find file");

    match args.action {
        Add { text, .. } => task::add_task(file, Task::new(text)),
        List => task::list_tasks(file),
        Done { position } => task::complete_task(file, position),
    }
    .expect("Failed to perform action")
}
