use core::fmt;

use super::point::Point;
use anyhow::{Ok, Result};

pub type MapInput = Vec<Vec<char>>;

#[derive(Clone)]
pub struct Map {
    pub map: Vec<Vec<Point>>,
    starting_points: Vec<Point>,
}

impl Map {
    pub fn new<F: FnMut(char) -> bool>(input: &MapInput, start_point_rule: &mut F) -> Result<Self> {
        let map = Self::get_map_of_points(input, start_point_rule)?;
        let starting_points = Self::find_starting_points(&map)?;
        Ok(Self {
            map,
            starting_points,
        })
    }

    fn find_starting_points(map: &Vec<Vec<Point>>) -> Result<Vec<Point>> {
        let mut starting_points = Vec::new();
        for (_i, elem) in map.iter().enumerate() {
            for (_j, p) in elem.iter().enumerate() {
                if p.is_starting_point {
                    starting_points.push(*p);
                }
            }
        }

        Ok(starting_points)
    }

    fn get_map_of_points<F: FnMut(char) -> bool>(
        input: &MapInput,
        start_point_rule: &mut F,
    ) -> Result<Vec<Vec<Point>>> {
        let map = input
            .iter()
            .enumerate()
            .map(|(i, e)| {
                e.iter()
                    .enumerate()
                    .map(|(j, f)| Point::new(j, i, *f, start_point_rule).unwrap())
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Vec<Point>>>();
        Ok(map)
    }

    pub fn clear_visited(&mut self) -> Result<()> {
        for row in self.map.iter_mut() {
            for p in row.iter_mut() {
                p.visited = false;
            }
        }
        Ok(())
    }

    pub fn get_starting_points(&self) -> Vec<Point> {
        self.starting_points.clone()
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
