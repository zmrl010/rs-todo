#![deny(missing_docs)]
//! Command line interface
//!
//! Using [`clap`] to do most of the legwork

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// [`Subcommand`]s for [`CommandLineArgs`]
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Activate a list by key. Commands following this
    /// one will be applied to that list
    Activate {
        #[arg()]
        key: String,
    },

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
    pub command: Commands,

    /// data directory where application state is stored
    ///
    /// # Default
    ///
    /// ```rust
    /// use std::path::PathBuf;
    ///
    /// PathBuf::from("~/.rs-todo/");
    /// ```
    #[arg(short, long, env)]
    pub data_dir: Option<PathBuf>,

    /// key of list that task commands are applied to
    ///
    /// # Default
    ///
    /// ```rust
    /// "[default]"
    /// ```
    #[arg(short, long, env)]
    pub list: Option<String>,
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
