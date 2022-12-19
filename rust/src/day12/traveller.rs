use super::{map::Map, point::Point};
use anyhow::{Ok, Result};
use core::fmt;

#[derive(Clone)]
pub struct Traveller {
    starting_point: Point,
    target_point: Point,
    spt: Vec<Point>,
    pub map: Map,
}

impl Traveller {
    pub fn new(map: Map) -> Result<Self> {
        let mut starting_point = Point::empty()?;
        let mut target_point = Point::empty()?;

        for (_i, elem) in map.map.iter().enumerate() {
            for (_j, p) in elem.iter().enumerate() {
                if p.is_starting_point.unwrap() {
                    starting_point = *p;
                } else if p.is_target_point.unwrap() {
                    target_point = *p;
                }
            }
        }

        let mut spt = Vec::new();
        spt.push(starting_point);

        Ok(Traveller {
            starting_point,
            target_point,
            spt,
            map,
        })
    }

    pub fn find_shortest_path_len(&mut self) -> Result<usize> {
        // mark started as visited
        self.map.map[self.starting_point.y][self.starting_point.x].visited = true;

        while !self.spt.is_empty() {
            let current_square = self.spt.remove(0);
            let mut next_squares = self.map.get_possible_moves(current_square);

            println!(
                "For point {:?} - checking: {:?}",
                &current_square, &next_squares
            );

            for s in &mut next_squares {
                if s.is_target_point.unwrap() {
                    // dbg!(&self.spt);
                    return Ok(current_square.distance.unwrap());
                } else {
                    self.spt.push(*s);
                };
            }
            self.spt.sort_by(|a, b| a.distance.cmp(&b.distance));
        }

        panic!("Target square not found")
    }
}

impl fmt::Debug for Traveller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;
        writeln!(
            f,
            "Starting Point ({}, {})",
            self.starting_point.x, self.starting_point.y
        )?;
        writeln!(
            f,
            "Target Point ({}, {})",
            self.target_point.x, self.target_point.y
        )?;
        writeln!(f, "")
    }
}
