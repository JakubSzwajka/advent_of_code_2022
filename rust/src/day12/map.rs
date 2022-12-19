use core::fmt;

use super::point::Point;
use anyhow::{Ok, Result};

pub type MapInput = Vec<Vec<Point>>;

#[derive(Clone)]
pub struct Map {
    pub map: MapInput,
}

impl Map {
    pub fn new(input: &MapInput) -> Result<Self> {
        Ok(Self { map: input.clone() })
    }

    pub fn get_possible_moves(&mut self, square: Point) -> Vec<Point> {
        let mut possible_moves = Vec::new();

        // left
        if square.x != 0
            && self.map[square.y][square.x - 1] <= square + 1
            && self.map[square.y][square.x - 1].visited == false
        {
            self.map[square.y][square.x - 1].distance = square.distance + 1;
            self.map[square.y][square.x - 1].visited = true;
            possible_moves.push(self.map[square.y][square.x - 1].clone());
        }
        // right
        if square.x < self.map[0].len() - 1
            && self.map[square.y][square.x + 1] <= square + 1
            && self.map[square.y][square.x + 1].visited == false
        {
            self.map[square.y][square.x + 1].distance = square.distance + 1;
            self.map[square.y][square.x + 1].visited = true;
            possible_moves.push(self.map[square.y][square.x + 1].clone());
        }
        // top
        if square.y != 0
            && self.map[square.y - 1][square.x] <= square + 1
            && self.map[square.y - 1][square.x].visited == false
        {
            self.map[square.y - 1][square.x].distance = square.distance + 1;
            self.map[square.y - 1][square.x].visited = true;
            possible_moves.push(self.map[square.y - 1][square.x].clone());
        }
        // bottom
        if square.y < self.map.len() - 1
            && self.map[square.y + 1][square.x] <= square + 1
            && self.map[square.y + 1][square.x].visited == false
        {
            self.map[square.y + 1][square.x].distance = square.distance + 1;
            self.map[square.y + 1][square.x].visited = true;
            possible_moves.push(self.map[square.y + 1][square.x].clone());
        }

        return possible_moves;
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "")?;
        for i in &self.map {
            let row = i
                .iter()
                .map(|e| e.h.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            writeln!(f, "{}", row)?;
        }
        writeln!(f, "")
    }
}
