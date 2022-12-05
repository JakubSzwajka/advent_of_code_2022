pub mod common;

fn main() {
    let args = common::read_args();
    let pairs = common::read_file(&args[1]);
}
