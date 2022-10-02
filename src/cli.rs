use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Add a task to the todo list
    Add {
        /// Task description text
        #[arg()]
        task: String,
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
    action: Action,

    /// File path to store todo list
    #[arg(short, long)]
    file: Option<PathBuf>,
}
