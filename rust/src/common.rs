use std::cmp::PartialOrd;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::iter::Sum;

pub fn read_file(file_path: &String) -> Vec<Result<String, std::io::Error>> {
    // return fs::read_to_string(file_path).expect("Should have been able to read the file");
    return BufReader::new(File::open(file_path).unwrap())
        .lines()
        .collect::<Vec<_>>();
}

pub fn read_args() -> Vec<String> {
    return env::args().collect();
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct VectorInfo<T> {
    pub max: (T, usize),
    pub min: (T, usize),
    pub sum: u64,
}

pub fn get_vector_info<T: Copy + PartialOrd>(vector: &[T]) -> VectorInfo<T> {
    let mut max = &vector[0];
    let mut min = &vector[0];
    let mut max_index = 0;
    let mut min_index = 0;
    // let mut sum: u64 = 0;

    for index in 1..vector.len() {
        if vector[index] < *min {
            min = &vector[index];
            min_index = index;
        }
        if vector[index] > *max {
            max = &vector[index];
            max_index = index;
        }
        // sum = sum + vector[index];
    }

    // let sum: &T = vector.iter().sum();

    VectorInfo {
        max: (*max, max_index),
        min: (*min, min_index),
        sum: 0,
    }
}
