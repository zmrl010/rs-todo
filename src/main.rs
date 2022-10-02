mod cli;
mod task;

use clap::Parser;
use cli::CommandLineArgs;

fn main() {
    let cli = CommandLineArgs::parse();
    println!("{:#?}", cli);
}
