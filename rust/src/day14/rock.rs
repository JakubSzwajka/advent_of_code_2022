use anyhow::Result;
use itertools::Itertools;

use super::point::Point;

pub const ROCK_SIGN: char = '#';

#[derive(Debug, Clone)]
pub struct Rock {
    line: Vec<Point>,
}

impl Rock {
    pub fn new(data: &str) -> Self {
        let points = data
            .split("->")
            .map(|x| x.parse::<Point>().unwrap())
            .collect_vec();
        Self { line: points }
    }

    pub fn get_rock_coordinates(&self) -> Result<Vec<Point>> {
        let mut rock_coordinates: Vec<Point> = Vec::new();

        for i in 0..self.line.len() - 1 {
            let start_point = &self.line[i];
            let stop_point = &self.line[i + 1];
            let mut points_between = start_point.get_points_between(stop_point)?;
            rock_coordinates.append(&mut points_between);
        }
        rock_coordinates.append(&mut self.line.clone());
        Ok(rock_coordinates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rock_coordinates_vertical() -> Result<()> {
        let rock = Rock::new("498,4 -> 498,5");
        assert!(rock.get_rock_coordinates()? == vec![Point::new(498, 4), Point::new(498, 5)]);
        Ok(())
    }
    #[test]
    fn test_get_rock_coordinates_horizontal() -> Result<()> {
        let rock = Rock::new("498,4 -> 499,4");
        assert!(rock.get_rock_coordinates()? == vec![Point::new(498, 4), Point::new(499, 4)]);
        Ok(())
    }
    #[test]
    fn test_get_rock_coordinates_mixed() -> Result<()> {
        let rock = Rock::new("498,4 -> 499,4 -> 499,5");
        assert!(
            rock.get_rock_coordinates()?
                == vec![Point::new(498, 4), Point::new(499, 4), Point::new(499, 5)]
        );
        Ok(())
    }
}
