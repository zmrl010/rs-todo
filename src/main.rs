use rs_todo::{self, CommandLineArgs, Parser};

fn main() -> rs_todo::Result<()> {
    let args = CommandLineArgs::parse();
    rs_todo::run(args)
}
