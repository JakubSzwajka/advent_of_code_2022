use crate::{
    day15::sensor::Sensor,
    solution::{Solution, SolutionInput},
    xlib::point::Point,
};
use anyhow::{Ok, Result};
mod sensor;
pub struct Day15Pt1;

impl Solution for Day15Pt1 {
    const DAY: usize = 15;
    const PART: usize = 1;

    type TInput = Vec<Sensor>;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let mut result = 0;
        let y_coordinate: isize = 2000000;

        let beacons = input
            .iter()
            .map(|x| x.beacon.clone())
            .filter(|x| x.y.eq(&y_coordinate))
            .collect::<Vec<Point<isize>>>();

        for i in -200000..5000000 {
            let p = Point::new(i, y_coordinate);
            for s in input {
                if s.does_contain(&p)? && !beacons.contains(&p) {
                    result += 1;
                    break;
                }
            }
        }
        Ok(result)
    }
}

pub struct Day15Pt2;

impl Solution for Day15Pt2 {
    const DAY: usize = 15;
    const PART: usize = 2;

    type TInput = Vec<Sensor>;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        fn calculate_tuning_frequency(p: &Point<isize>) -> Result<usize> {
            Ok(p.x as usize * 4000000 + p.y as usize)
        }

        let y_index: isize = 4000000;
        // let y_index: isize = 20;

        let candidates = input
            .iter()
            .map(|x| {
                x.get_adjacent()
                    .unwrap()
                    .into_iter()
                    .filter(|p| p.x > 0 && p.x <= y_index && p.y > 0 && p.y <= y_index)
                    .collect()
            })
            .collect::<Vec<Vec<Point<isize>>>>()
            .concat();

        for (i, p) in candidates.iter().enumerate() {
            println!("Candidate {}/{}", i, candidates.len());
            let mut does_contain = false;

            for sensor in input {
                if sensor.does_contain(&p)? {
                    does_contain = true;
                    break;
                }
            }
            if !does_contain {
                println!("Found beacon: ({},{})", p.x, p.y);
                return Ok(calculate_tuning_frequency(&p)?);
            }
        }
        Ok(1)
    }
}

impl SolutionInput for Vec<Sensor> {
    fn parse(input_str: &str) -> Result<Self> {
        let sensors = input_str
            .split("\n")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<Sensor>>();
        Ok(sensors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Vec<Sensor> = get_input::<Day15Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Vec<Sensor> = get_input::<Day15Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(4811413, Day15Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part2_result() -> Result<()> {
        assert_eq!(13171855019123, Day15Pt2::solve(&INPUT_MAIN)?);
        Ok(())
    }
}
