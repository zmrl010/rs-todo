use rs_todo;

fn main() -> anyhow::Result<()> {
    let args = rs_todo::parse();
    rs_todo::run(args)
}
