use anyhow::Result;

mod common;
mod day11;
mod day12;
mod solution;

use crate::solution::Solution;

use day11::{Day11Pt1, Day11Pt2};
use day12::{Day12Pt1, Day12Pt2};

fn main() -> Result<()> {
    Day11Pt1::run()?;
    Day11Pt2::run()?;
    Day12Pt1::run()?;
    Day12Pt2::run()?;
    Ok(())
}
