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
        fn search_row(
            sensors: &Vec<Sensor>,
            beacons: &Vec<Point<isize>>,
            y_index: isize,
            offset: isize,
        ) -> Option<Point<isize>> {
            for i in 0..(y_index * 2) + 1 {
                let mut new_beacon_found = true;
                let mut beacon: Option<Point<isize>> = None;
                let p = Point::new(i, y_index + offset);
                // println!("Checking: {}/{}", p.x, p.y);
                if !beacons.contains(&p) {
                    for (_i, s) in sensors.iter().enumerate() {
                        if !s.does_contain(&p).unwrap() {
                            // println!(
                            //     "{} - Not found in S({}/{})",
                            //     _i, s.coordinates.x, s.coordinates.y
                            // );
                            beacon = Some(p.clone());
                        } else {
                            new_beacon_found = false;
                            // println!("--> Found in S({}/{})", s.coordinates.x, s.coordinates.y,);
                            break;
                        }
                    }
                    if new_beacon_found {
                        return beacon;
                    }
                }
            }
            return None;
        }

        fn calculate_tuning_frequency(p: &Point<isize>) -> Result<usize> {
            Ok(p.x as usize * 4000000 + p.y as usize)
        }

        let y_index: isize = 2000000;
        // let y_index: isize = 10;

        let range = MyRange::new(y_index);

        let beacons = input
            .iter()
            .map(|x| x.beacon.clone())
            .collect::<Vec<Point<isize>>>();
        let mut closest_beacon: Point<isize> = "0,0".parse().unwrap();

        println!("Checking!");
        for i in range {
            println!("Checking row {}/{}", i, (y_index * 2) + 1);
            match search_row(&input, &beacons, y_index as isize, i as isize) {
                Some(p) => {
                    if (y_index - p.y).abs() < (y_index - closest_beacon.y).abs() {
                        closest_beacon = p;
                    }
                }
                None => {}
            }
        }

        // dbg!(&closest_beacon);
        Ok(calculate_tuning_frequency(&closest_beacon)?)
        // Ok(1)
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

    // #[test]
    // fn test_part1_test() -> Result<()> {
    //     assert_eq!(26, Day15Pt1::solve(&INPUT_TEST)?);
    //     Ok(())
    // }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(4811413, Day15Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    // #[test]
    // fn test_part2_test() -> Result<()> {
    //     assert_eq!(56000011, Day15Pt2::solve(&INPUT_TEST)?);
    //     Ok(())
    // }

    // #[test]
    // fn test_part2_result() -> Result<()> {
    //     assert_eq!(332, Day15Pt2::solve(&INPUT_MAIN)?);
    //     Ok(())
    // }
}

#[derive(Debug)]
struct MyRange {
    start: isize,
    offset: isize,
    direction: bool, // true for higher / false for lower ( to 0 )
}

impl MyRange {
    fn new(start: isize) -> MyRange {
        MyRange {
            start,
            offset: 1,
            direction: true,
        }
    }
}

impl Iterator for MyRange {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.start {
            None
        } else {
            if self.direction {
                let result = self.start + self.offset;
                self.direction = false;
                Some(result)
            } else {
                let result = self.start - self.offset;
                self.direction = true;
                self.offset += 1;
                Some(result)
            }
        }
    }
}
