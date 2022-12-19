use anyhow::{Ok, Result};
use core::fmt;
use std::ops;

pub const START_POS_MARK: char = 'S';
pub const END_POS_MARK: char = 'E';

#[derive(Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub h: usize,
    pub is_starting_point: bool,
    pub is_target_point: bool,
    pub visited: bool,
    pub distance: usize,
}

impl Point {
    pub fn new(x: usize, y: usize, h: char) -> Result<Self> {
        Ok(Point {
            x: x,
            y: y,
            h: Self::get_height(h),
            is_starting_point: h == START_POS_MARK,
            is_target_point: h == END_POS_MARK,
            distance: 1,
            visited: false,
        })
    }

    fn get_height(h: char) -> usize {
        match h {
            START_POS_MARK => 'a' as usize - 58,
            END_POS_MARK => 'z' as usize - 58,
            _ => {
                let height_digit = h as usize;
                if height_digit > 90 {
                    height_digit - 58
                } else {
                    height_digit
                }
            }
        }
    }

    pub fn empty() -> Result<Self> {
        Ok(Point {
            x: 0,
            y: 0,
            h: 0,
            is_starting_point: false,
            is_target_point: false,
            distance: 0,
            visited: false,
        })
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({},{}) dst:{} height:{}",
            self.x, self.y, self.distance, self.h
        )
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.h == other.h
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
            h: self.h + rhs,
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
        let point_a = Point::new(0, 0, 'a')?;
        let point_b = Point::new(312, 12, 'a')?;
        assert_eq!(point_a, point_b);
        Ok(())
    }

    #[test]
    fn test_a_point_is_grater_then_b() -> Result<()> {
        let point_a = Point::new(1, 31, 'A')?;
        let point_b = Point::new(312, 12, 'a')?;
        assert_eq!(true, point_a >= point_b);
        Ok(())
    }

    #[test]
    fn test_adding_usize_increase_h() -> Result<()> {
        let point_a = Point::new(1, 31, 'a')?;
        let bigger_point = point_a + 100;
        assert_eq!(139, bigger_point.h);
        Ok(())
    }
}
