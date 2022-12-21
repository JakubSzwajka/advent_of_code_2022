use anyhow::Result;

mod common;
mod day11;
mod day12;
mod day13;
mod day14;
mod solution;

use crate::solution::Solution;

use day11::{Day11Pt1, Day11Pt2};
use day12::{Day12Pt1, Day12Pt2};
use day13::{Day13Pt1, Day13Pt2};
use day14::Day14Pt1;

fn main() -> Result<()> {
    // Day11Pt1::run()?;
    // Day11Pt2::run()?;
    // Day12Pt1::run()?;
    // Day12Pt2::run()?;
    // Day13Pt1::run()?;
    // Day13Pt2::run()?;
    Day14Pt1::run()?;
    Ok(())
}
