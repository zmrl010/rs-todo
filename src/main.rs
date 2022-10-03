use clap::Parser;

use rs_todo::{self, cli::CommandLineArgs};

fn main() -> anyhow::Result<()> {
    let args = CommandLineArgs::parse();
    rs_todo::run(args)
}
