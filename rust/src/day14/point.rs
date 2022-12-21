use anyhow::{Error, Result};
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }

    fn get_vertical_points_to(&self, other: &Self) -> Result<Vec<Point>> {
        let mut points: Vec<Self> = Vec::new();
        let mut i = self.y + 1;
        while i < other.y {
            points.push(Point::new(self.x, i));
            i += 1;
        }
        Ok(points)
    }
    fn get_horizontal_points_to(&self, other: &Self) -> Result<Vec<Point>> {
        let mut points: Vec<Self> = Vec::new();
        let mut i = self.x + 1;
        while i < other.x {
            points.push(Point::new(i, self.y));
            i += 1;
        }
        Ok(points)
    }

    pub fn get_points_between(&self, other: &Self) -> Result<Vec<Point>> {
        if self.x == other.x {
            if self.y < other.y {
                Ok(self.get_vertical_points_to(other)?)
            } else {
                Ok(other.get_vertical_points_to(self)?)
            }
        } else if self.y == other.y {
            if self.x < other.x {
                Ok(self.get_horizontal_points_to(other)?)
            } else {
                Ok(other.get_horizontal_points_to(self)?)
            }
        } else {
            panic!("Points not diagonal")
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == self.y
    }
}

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x_y = s
            .trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect_tuple::<(usize, usize)>()
            .unwrap();
        Ok(Self::new(x_y.0, x_y.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_points_between_vertical() -> Result<()> {
        let a = Point::new(1, 1);
        let b = Point::new(1, 3);
        assert!(a.get_points_between(&b)? == vec![Point::new(1, 2)]);
        assert!(b.get_points_between(&a)? == vec![Point::new(1, 2)]);
        Ok(())
    }
    #[test]
    fn test_get_points_between_horizontal() -> Result<()> {
        let a = Point::new(1, 1);
        let b = Point::new(3, 1);
        assert!(a.get_points_between(&b)? == vec![Point::new(2, 1)]);
        assert!(b.get_points_between(&a)? == vec![Point::new(2, 1)]);
        Ok(())
    }
}
