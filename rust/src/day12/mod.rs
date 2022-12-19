mod map;
mod point;
mod traveller;

use crate::solution::{Solution, SolutionInput};
use anyhow::{Ok, Result};
use map::{Map, MapInput};
use point::Point;
use traveller::Traveller;

pub struct Day12Pt1;

impl Solution for Day12Pt1 {
    const DAY: usize = 12;
    const PART: usize = 1;

    type TInput = MapInput;
    type TOutput = usize;
    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let map = Map::new(input)?;
        let mut traveller = Traveller::new(map)?;
        let len = traveller.find_shortest_path_len()?;
        Ok(len)
    }
}

impl SolutionInput for Vec<Vec<Point>> {
    fn parse(input_str: &str) -> Result<Self> {
        let points_height = input_str
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.chars().map(|c| c).collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let points_obj = points_height
            .iter()
            .enumerate()
            .map(|(i, e)| {
                e.iter()
                    .enumerate()
                    .map(|(j, f)| Point::new(j, i, Some(*f)).unwrap())
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Vec<Point>>>();
        Ok(points_obj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Vec<Vec<Point>> = get_input::<Day12Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Vec<Vec<Point>> = get_input::<Day12Pt1>("input.txt").unwrap();
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

    // #[test]
    // fn test_part2_test() -> Result<()> {
    //     assert_eq!(2713310158, Day11Pt2::solve(&INPUT_TEST)?);
    //     Ok(())
    // }
    // #[test]
    // fn test_part2_result() -> Result<()> {
    //     assert_eq!(29703395016, Day11Pt2::solve(&INPUT_MAIN)?);
    //     Ok(())
    // }
}
