use anyhow::{Error, Result};
use itertools::Itertools;
use num::{abs, Signed};
use num_traits::{Num, PrimInt};
use std::{
    fmt::{self, Debug},
    ops::AddAssign,
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct Point<T>
where
    T: Num + Ord + AddAssign + Copy + PrimInt + Signed,
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Num + Ord + AddAssign + Copy + PrimInt + Signed,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn get_vertical_points_to(&self, other: &Self) -> Result<Vec<Point<T>>> {
        let mut points: Vec<Self> = Vec::new();
        let mut i = self.y + T::one();
        while i < other.y {
            points.push(Point::new(self.x, i));
            i += T::one();
        }
        Ok(points)
    }
    fn get_horizontal_points_to(&self, other: &Self) -> Result<Vec<Point<T>>> {
        let mut points: Vec<Self> = Vec::new();
        let mut i = self.x + T::one();
        while i < other.x {
            points.push(Point::new(i, self.y));
            i += T::one();
        }
        Ok(points)
    }

    pub fn get_points_between(&self, other: &Self) -> Result<Vec<Point<T>>> {
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

    pub fn get_distance_to(&self, other: &Self) -> Result<f32> {
        let d = (self.x - other.x).pow(2) + (self.y - other.y).pow(2);
        let distance = T::to_f32(&d).unwrap().sqrt();
        Ok(distance)
    }

    pub fn get_manhattan_distance_to(&self, other: &Self) -> Result<T> {
        Ok(abs(self.x - other.x) + abs(self.y - other.y))
    }
}

impl<T: Num + Ord + AddAssign + Copy + PrimInt + Signed> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == self.y
    }
}

impl<T> FromStr for Point<T>
where
    T: Num + FromStr + Ord + AddAssign + Copy + PrimInt + Signed,
    // Every FromStr implementation has to define
    // the associated error type, Err. However, there are
    // no constrains on Err. It might implement Debug, but it
    // also might not. Since our trait bounds on test_parse say
    // that we're fine to run over all Ts that implement
    // just FromStr, we're too generic.
    // We need to say that test_parse will only work for Ts that
    // are FromStr and whose associated Err type can be shown
    // via Debug
    <T as FromStr>::Err: fmt::Debug,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x_y = s
            .trim()
            .split(",")
            .map(|x| x.parse::<T>().unwrap())
            .collect_tuple::<(T, T)>()
            .unwrap();
        Ok(Self::new(x_y.0, x_y.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_from_string() -> Result<()> {
        let p: Point<isize> = "123,321".parse().unwrap();
        assert_eq!(p, Point::new(123, 321));
        let p: Point<isize> = "2,18".parse().unwrap();
        assert_eq!(p, Point::new(2, 18));
        let p: Point<isize> = "-2,15".parse().unwrap();
        assert_eq!(p, Point::new(-2, 18));
        Ok(())
    }

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

    #[test]
    fn test_get_distance() -> Result<()> {
        let a = Point::new(1, 1);
        let b = Point::new(3, 1);
        assert_eq!(a.get_distance_to(&b)?, 2.0);
        Ok(())
    }

    #[test]
    fn test_get_manhattan_distance() -> Result<()> {
        let a = Point::new(1, 1);
        let b = Point::new(3, 1);
        assert_eq!(a.get_manhattan_distance_to(&b)?, 2);

        let a = Point::new(1, 1);
        let b = Point::new(3, 3);
        assert_eq!(a.get_manhattan_distance_to(&b)?, 4);
        Ok(())
    }
}
