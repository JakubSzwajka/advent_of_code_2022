pub mod common;

fn main() {
    let args = common::read_args();
    let starting_cargo_data = common::read_file(&args[1]).unwrap();
}
