use crate::xlib::point::Point;
use anyhow::{Error, Ok, Result};
use std::str::FromStr;

type TPoint = isize;

#[derive(Debug, Clone)]
pub struct Sensor {
    pub coordinates: Point<TPoint>,
    pub beacon: Point<TPoint>,
}

impl Sensor {
    fn new(coordinates: Point<TPoint>, beacon: Point<TPoint>) -> Self {
        Self {
            coordinates,
            beacon,
        }
    }

    pub fn does_contain(&self, p: &Point<TPoint>) -> Result<bool> {
        Ok(self.coordinates.get_manhattan_distance_to(&self.beacon)?
            >= self.coordinates.get_manhattan_distance_to(p)?)
    }

    pub fn get_adjacent(&self) -> Result<Vec<Point<isize>>> {
        let mut res: Vec<Point<isize>> = Vec::new();
        let r = self.coordinates.get_manhattan_distance_to(&self.beacon)? + 1;

        let x = self.coordinates.x;
        let y = self.coordinates.y;
        let mut x_offsize = 0;
        let mut y_offsize = 0;

        // left to top
        while x - r + x_offsize != self.coordinates.x {
            res.push(Point::new(x - r + x_offsize, y + y_offsize));
            x_offsize += 1;
            y_offsize -= 1;
        }
        // top to right
        x_offsize = 0;
        y_offsize = 0;

        while x + x_offsize != self.coordinates.x + r {
            res.push(Point::new(x + x_offsize, y - r + y_offsize));
            x_offsize += 1;
            y_offsize += 1;
        }

        // right to bottom
        x_offsize = 0;
        y_offsize = 0;

        while x + r + x_offsize != self.coordinates.x {
            res.push(Point::new(x + r + x_offsize, y + y_offsize));
            x_offsize -= 1;
            y_offsize += 1;
        }

        // right to left
        x_offsize = 0;
        y_offsize = 0;

        while x + x_offsize > self.coordinates.x - r {
            res.push(Point::new(x + x_offsize, y + r + y_offsize));
            x_offsize -= 1;
            y_offsize -= 1;
        }

        Ok(res)
    }
}

impl FromStr for Sensor {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn get_xy(s: &str) -> Result<String> {
            let mut x: Option<&str> = None;
            let mut y: Option<&str> = None;
            for part in s.split(" ") {
                if part.starts_with("x=") {
                    x = Some(&part[2..part.len() - 1]);
                } else if part.starts_with("y=") {
                    y = Some(&part[2..]);
                }
            }
            let xy = format!("{},{}", x.unwrap(), y.unwrap());
            Ok(xy)
        }

        let z = s.split(":").collect::<Vec<&str>>();
        let sensor_part: Point<TPoint> = get_xy(z[0])?.parse()?;
        let beacon_part: Point<TPoint> = get_xy(z[1])?.parse()?;

        Ok(Self::new(sensor_part, beacon_part))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() -> Result<()> {
        let s: Sensor = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".parse()?;
        assert_eq!(s.beacon.x, -2);
        assert_eq!(s.beacon.y, 15);
        assert_eq!(s.coordinates.x, 2);
        assert_eq!(s.coordinates.y, 18);
        Ok(())
    }

    #[test]
    fn test_contain_point() -> Result<()> {
        let s: Sensor = "Sensor at x=8, y=7: closest beacon is at x=2, y=10".parse()?;
        assert!(s.does_contain(&"8,16".parse::<Point<TPoint>>()?)?);
        assert!(s.does_contain(&"5,9".parse::<Point<TPoint>>()?)?);
        assert!(!s.does_contain(&"8,17".parse::<Point<TPoint>>()?)?);
        assert!(!s.does_contain(&"2,11".parse::<Point<TPoint>>()?)?);
        Ok(())
    }

    #[test]
    fn test_adjacent_points() -> Result<()> {
        let s: Sensor = "Sensor at x=12, y=2: closest beacon is at x=12, y=3".parse()?;
        let adjacent = s.get_adjacent()?;
        // dbg!(&adjacent);

        assert_eq!(adjacent.len(), 8);
        assert!(adjacent.contains(&Point::new(10, 2)));
        assert!(adjacent.contains(&Point::new(11, 1)));
        assert!(adjacent.contains(&Point::new(12, 0)));
        assert!(adjacent.contains(&Point::new(13, 1)));
        assert!(adjacent.contains(&Point::new(14, 2)));
        assert!(adjacent.contains(&Point::new(13, 3)));
        assert!(adjacent.contains(&Point::new(12, 4)));
        assert!(adjacent.contains(&Point::new(11, 3)));
        Ok(())
    }
}
