mod map;
mod rock;
mod sand;
use crate::solution::{Solution, SolutionInput};
use anyhow::{Ok, Result};
use map::Map;
use rock::Rock;

pub struct Day14Pt1;

impl Solution for Day14Pt1 {
    const DAY: usize = 14;
    const PART: usize = 1;

    type TInput = Vec<Rock>;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let mut map = Map::new();
        for rock in input.to_vec() {
            map.add_rock(rock)?;
        }

        let mut units_of_sand = 0;
        let mut endless_void_reached = false;
        while !endless_void_reached {
            if let Err(_) = map.produce_sand() {
                endless_void_reached = true
            } else {
                units_of_sand += 1;
            }
        }

        // dbg!(&map);
        Ok(units_of_sand)
    }
}
pub struct Day14Pt2;

impl Solution for Day14Pt2 {
    const DAY: usize = 14;
    const PART: usize = 2;

    type TInput = Vec<Rock>;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let mut map = Map::new();
        for rock in input.to_vec() {
            map.add_rock(rock)?;
        }
        map.add_floor()?;

        let mut units_of_sand = 0;
        let mut entrance_blocked = false;
        while !entrance_blocked {
            if let Err(_) = map.produce_sand() {
                entrance_blocked = true
            } else {
                units_of_sand += 1;
            }
        }

        dbg!(&map);
        Ok(units_of_sand)
    }
}

impl SolutionInput for Vec<Rock> {
    fn parse(input_str: &str) -> Result<Self> {
        let rocks = input_str
            .split("\n")
            .map(|x| Rock::new(x))
            .collect::<Vec<Rock>>();

        Ok(rocks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Vec<Rock> = get_input::<Day14Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Vec<Rock> = get_input::<Day14Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_part1_test() -> Result<()> {
        assert_eq!(24, Day14Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(578, Day14Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part2_test() -> Result<()> {
        assert_eq!(93, Day14Pt2::solve(&INPUT_TEST)?);
        Ok(())
    }
    #[test]
    fn test_part2_result() -> Result<()> {
        assert_eq!(24377, Day14Pt2::solve(&INPUT_MAIN)?);
        Ok(())
    }
}
