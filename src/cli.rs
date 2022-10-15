#![deny(missing_docs)]
//! Command line interface
//!
//! Using [`clap`] to do most of the legwork

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
/// Commands for [`crate::tasks::Task`]
pub enum TaskCommand {
    /// Add a task to the todo list
    Add {
        #[arg()]
        /// Task description text
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
/// Command-line interface for rs-todo
pub struct CommandLineArgs {
    /// task subcommand to act on active list
    #[command(subcommand)]
    pub command: TaskCommand,
    /// data directory where application state is stored
    #[arg(short, long, env)]
    pub data_dir: Option<PathBuf>,
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
