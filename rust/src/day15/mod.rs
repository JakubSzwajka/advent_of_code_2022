use crate::solution::{Solution, SolutionInput};
use anyhow::{Ok, Result};
pub struct Day15Pt1;

impl Solution for Day15Pt1 {
    const DAY: usize = 15;
    const PART: usize = 1;

    type TInput = usize;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        Ok(1)
    }
}

impl SolutionInput for usize {
    fn parse(input_str: &str) -> Result<Self> {
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: usize = get_input::<Day15Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: usize = get_input::<Day15Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_part1_test() -> Result<()> {
        assert_eq!(31, Day15Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }

    // #[test]
    // fn test_part1_result() -> Result<()> {
    //     assert_eq!(339, Day13Pt1::solve(&INPUT_MAIN)?);
    //     Ok(())
    // }

    // #[test]
    // fn test_part2_test() -> Result<()> {
    //     assert_eq!(29, Day12Pt2::solve(&INPUT_TEST)?);
    //     Ok(())
    // }
    // #[test]
    // fn test_part2_result() -> Result<()> {
    //     assert_eq!(332, Day12Pt2::solve(&INPUT_MAIN)?);
    //     Ok(())
    // }
}
