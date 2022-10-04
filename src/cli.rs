use std::path::PathBuf;

pub use clap::Parser;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum TaskCommand {
    /// Add a task to the todo list
    Add {
        /// Task description text
        #[arg()]
        text: String,
    },
    /// Remove a task from the todo list
    Done {
        #[arg()]
        position: usize,
    },
    /// List all tasks in a todo list
    List,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArgs {
    #[command(subcommand)]
    pub command: TaskCommand,

    /// Directory to task lists
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    CommandLineArgs::command().debug_assert()
}
