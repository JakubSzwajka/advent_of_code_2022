use self::packet::Packet;
use crate::solution::{Solution, SolutionInput};
use anyhow::{Ok, Result};
use itertools::Itertools;
mod packet;
pub struct Day13Pt1;
pub struct Day13Pt2;

type VecPacketPairs = Vec<(Packet, Packet)>;
type VecPacket = Vec<Packet>;

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

impl Solution for Day13Pt2 {
    const DAY: usize = 13;
    const PART: usize = 2;

    type TInput = VecPacket;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        dbg!(input);
        let mut packets = input.to_vec();

        let mut dividers = vec![
            Packet::List(vec![Packet::Int(2)]),
            Packet::List(vec![Packet::Int(6)]),
        ];

        packets.append(&mut dividers);
        packets.sort();

        // +1 since positions are numered from 0
        let div1_pos = packets
            .iter()
            .position(|x| *x == Packet::List(vec![Packet::Int(2)]))
            .unwrap()
            + 1;
        let div2_pos = packets
            .iter()
            .position(|x| *x == Packet::List(vec![Packet::Int(6)]))
            .unwrap()
            + 1;

        dbg!(div1_pos, div2_pos);
        Ok(div1_pos * div2_pos)
    }
}

impl SolutionInput for VecPacket {
    fn parse(input_str: &str) -> Result<Self> {
        let packets = input_str
            .split("\n")
            .filter(|x| *x != "")
            .map(|x| x.parse::<Packet>().unwrap())
            .collect_vec();
        Ok(packets)
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
        static ref INPUT_TEST_PT1: VecPacketPairs = get_input::<Day13Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN_PT1: VecPacketPairs = get_input::<Day13Pt1>("input.txt").unwrap();
        static ref INPUT_TEST_PT2: VecPacket = get_input::<Day13Pt2>("test.txt").unwrap();
        static ref INPUT_MAIN_PT2: VecPacket = get_input::<Day13Pt2>("input.txt").unwrap();
    }

    #[test]
    fn test_part1_test() -> Result<()> {
        assert_eq!(13, Day13Pt1::solve(&INPUT_TEST_PT1)?);
        Ok(())
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(5843, Day13Pt1::solve(&INPUT_MAIN_PT1)?);
        Ok(())
    }

    #[test]
    fn test_part2_test() -> Result<()> {
        assert_eq!(140, Day13Pt2::solve(&INPUT_TEST_PT2)?);
        Ok(())
    }

    #[test]
    fn test_part2_result() -> Result<()> {
        assert_eq!(26289, Day13Pt2::solve(&INPUT_MAIN_PT2)?);
        Ok(())
    }
}
