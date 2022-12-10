pub mod common;

fn main() {
    let args = common::read_args();
    let commands = common::read_file(&args[1]).unwrap();
}
