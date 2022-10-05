use rs_todo::{
    self,
    cli::{CommandLineArgs, Parser},
};

fn main() -> anyhow::Result<()> {
    let args = CommandLineArgs::parse();
    rs_todo::run(args)
}
