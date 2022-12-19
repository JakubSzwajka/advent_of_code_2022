use anyhow::{Context, Result};
use std::cmp::PartialOrd;
use std::env;
use std::io::{BufRead, BufReader, Read};

use crate::solution::{Solution, SolutionInput};

use std::{fmt::Debug, fs, fs::File};

pub fn read_file_old(file_path: &String) -> Vec<Result<String, std::io::Error>> {
    BufReader::new(File::open(file_path).unwrap())
        .lines()
        .collect::<Vec<_>>()
}

pub fn read_file(file_path: &String) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn read_args() -> Vec<String> {
    env::args().collect()
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

pub fn get_input<T: Solution + ?Sized>(name: &str) -> Result<T::TInput> {
    let file_name = format!("src/Day{}/{}", T::DAY, name);
    let input_str = fs::read_to_string(&file_name).context(format!("reading {:?}", file_name))?;
    T::TInput::parse(&input_str).context(format!("parsing {:?}", &file_name))
}
