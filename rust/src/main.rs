use anyhow::Result;

mod common;
mod day11;
mod solution;

use crate::solution::Solution;

use day11::{Day11Pt1, Day11Pt2};

fn main() -> Result<()> {
    Day11Pt1::run()?;
    Day11Pt2::run()?;
    Ok(())
}
