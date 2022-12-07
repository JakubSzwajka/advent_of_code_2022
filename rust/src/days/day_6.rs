use std::collections::HashSet;

pub mod common;

fn has_unique_elements(c: &[char]) -> bool {
    let mut uniq = HashSet::new();
    c.into_iter().all(move |x| uniq.insert(x))
}

fn main() {
    let args = common::read_args();
    let signal = common::read_file(&args[1]).unwrap();

    let char_signal: Vec<char> = signal.chars().collect();
    for i in 14..char_signal.len() {
        if has_unique_elements(&char_signal[i - 14..i]) {
            dbg!(&char_signal[i - 14..i]);
            dbg!(i);
            break;
        }
    }
}
