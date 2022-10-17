#![deny(missing_docs)]
//! Command line interface
//!
//! Using [`clap`] to do most of the legwork

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Commands for [`crate::tasks::Task`]
#[derive(Subcommand, Debug)]
pub enum TaskCommand {
    /// Add a task to the todo list
    Add {
        /// Task description text
        #[arg()]
        text: String,
    },

    /// Mark a task as complete
    Done {
        /// Index position of task to complete
        #[arg()]
        position: usize,
    },

    /// List all tasks in todo list
    List,
}

/// rs-todo - a simple task manager
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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
