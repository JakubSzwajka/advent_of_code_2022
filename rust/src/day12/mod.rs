mod map;
mod point;
mod traveller;

use crate::solution::{Solution, SolutionInput};
use anyhow::{Ok, Result};
use map::{Map, MapInput};
use traveller::Traveller;

use self::point::LOWEST_POS_MARK;
use self::point::START_POS_MARK;

pub struct Day12Pt1;

impl Solution for Day12Pt1 {
    const DAY: usize = 12;
    const PART: usize = 1;

    type TInput = MapInput;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let map = Map::new(input, &mut |h: char| h == START_POS_MARK)?;
        let mut traveller = Traveller::new(map)?;
        let mut shortest_paths: Vec<usize> = Vec::new();

        for p in traveller.map.get_starting_points() {
            shortest_paths.push(traveller.find_shortest_from_point(p)?);
        }

        shortest_paths.sort();
        dbg!(&shortest_paths);

        Ok(shortest_paths[0])
    }
}

pub struct Day12Pt2;

impl Solution for Day12Pt2 {
    const DAY: usize = 12;
    const PART: usize = 2;

    type TInput = MapInput;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let map = Map::new(input, &mut |h: char| h == LOWEST_POS_MARK)?;
        let mut traveller = Traveller::new(map)?;
        let mut shortest_paths: Vec<usize> = Vec::new();

        for p in traveller.map.get_starting_points() {
            shortest_paths.push(traveller.find_shortest_from_point(p)?);
            traveller.map.clear_visited()?;
        }

        shortest_paths.sort();
        dbg!(&shortest_paths);

        Ok(shortest_paths[0])
    }
}

impl SolutionInput for Vec<Vec<char>> {
    fn parse(input_str: &str) -> Result<Self> {
        let points_height = input_str
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.chars().map(|c| c).collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Ok(points_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Vec<Vec<char>> = get_input::<Day12Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Vec<Vec<char>> = get_input::<Day12Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_part1_test() -> Result<()> {
        assert_eq!(31, Day12Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(339, Day12Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part2_test() -> Result<()> {
        assert_eq!(29, Day12Pt2::solve(&INPUT_TEST)?);
        Ok(())
    }
    #[test]
    fn test_part2_result() -> Result<()> {
        assert_eq!(332, Day12Pt2::solve(&INPUT_MAIN)?);
        Ok(())
    }
}
