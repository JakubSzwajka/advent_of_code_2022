use self::packet::Packet;
use crate::solution::{Solution, SolutionInput};
use anyhow::{Ok, Result};
use itertools::Itertools;
mod packet;
pub struct Day13Pt1;

type VecPacketPairs = Vec<(Packet, Packet)>;

impl Solution for Day13Pt1 {
    const DAY: usize = 13;
    const PART: usize = 1;

    type TInput = VecPacketPairs;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let mut correct_packages = 0;
        for (i, (left, right)) in input.iter().enumerate() {
            if left < right {
                // Added one since enumerate is from 0
                correct_packages += i + 1;
            }
        }
        Ok(correct_packages)
    }
}

impl SolutionInput for VecPacketPairs {
    fn parse(input_str: &str) -> Result<Self> {
        let packet_pairs = input_str
            .split("\n\n")
            .map(|x| {
                x.split("\n")
                    .map(|y| y.parse::<Packet>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect::<VecPacketPairs>();

        Ok(packet_pairs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: VecPacketPairs = get_input::<Day13Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: VecPacketPairs = get_input::<Day13Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_part1_test() -> Result<()> {
        assert_eq!(13, Day13Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(5843, Day13Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

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
