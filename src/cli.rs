//! Command line interface
//!
//! Using [`clap`] to do most of the legwork

use clap::{Parser, Subcommand};
use std::path::PathBuf;

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

    /// File path to list
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}

/// Parse from `std::env::args_os()`, exit on error
///
/// Convenient alias for [Parser::parse]
pub fn parse() -> CommandLineArgs {
    CommandLineArgs::parse()
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    CommandLineArgs::command().debug_assert()
}
