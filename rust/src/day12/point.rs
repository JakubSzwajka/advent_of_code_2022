use anyhow::{Ok, Result};
use core::fmt;
use std::ops;

pub const START_POS_MARK: char = 'S';
pub const END_POS_MARK: char = 'E';

#[derive(Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub h: Option<usize>,
    pub is_starting_point: Option<bool>,
    pub is_target_point: Option<bool>,
    pub visited: bool,
    pub distance: Option<usize>,
}

impl Point {
    pub fn new(x: usize, y: usize, h: Option<char>) -> Result<Self> {
        let height_digit = h.unwrap() as usize;
        if h == Some(START_POS_MARK) {
            Ok(Point {
                x: x,
                y: y,
                h: Some(('a' as usize) - 58),
                is_starting_point: Some(true),
                is_target_point: Some(false),
                distance: Some(1),
                visited: false,
            })
        } else if h == Some(END_POS_MARK) {
            Ok(Point {
                x: x,
                y: y,
                h: Some(('z' as usize) - 58),
                is_starting_point: Some(false),
                is_target_point: Some(true),
                distance: Some(1),
                visited: false,
            })
        } else {
            let height = if height_digit > 90 {
                height_digit - 58
            } else {
                height_digit
            };

            Ok(Point {
                x: x,
                y: y,
                h: Some(height),
                is_starting_point: Some(false),
                is_target_point: Some(false),
                distance: None,
                visited: false,
            })
        }
    }

    pub fn empty() -> Result<Self> {
        Ok(Point {
            x: 0,
            y: 0,
            h: None,
            is_starting_point: Some(false),
            is_target_point: Some(false),
            distance: None,
            visited: false,
        })
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}", self.h.unwrap())
        write!(
            f,
            "({},{}) dst:{} height:{}",
            self.x,
            self.y,
            self.distance.unwrap_or_default(),
            self.h.unwrap()
        )
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.h.unwrap() == other.h.unwrap()
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.h.cmp(&other.h))
    }
}

impl ops::Add<usize> for Point {
    type Output = Point;
    fn add(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x,
            y: self.y,
            h: Some(self.h.unwrap() + rhs),
            is_starting_point: self.is_starting_point,
            is_target_point: self.is_target_point,
            distance: self.distance,
            visited: self.visited,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points_are_eq() -> Result<()> {
        let point_a = Point::new(0, 0, Some('a'))?;
        let point_b = Point::new(312, 12, Some('a'))?;
        assert_eq!(point_a, point_b);
        Ok(())
    }

    #[test]
    fn test_a_point_is_grater_then_b() -> Result<()> {
        let point_a = Point::new(1, 31, Some('A'))?;
        let point_b = Point::new(312, 12, Some('a'))?;
        assert_eq!(true, point_a >= point_b);
        Ok(())
    }

    #[test]
    fn test_adding_usize_increase_h() -> Result<()> {
        let point_a = Point::new(1, 31, Some('a'))?;
        let bigger_point = point_a + 100;
        assert_eq!(139, bigger_point.h.unwrap());
        Ok(())
    }
}
