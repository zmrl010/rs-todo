use rs_todo;

fn main() -> rs_todo::Result<()> {
    let args = rs_todo::parse();
    rs_todo::run(args)
}
