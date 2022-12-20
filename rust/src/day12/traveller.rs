use super::{map::Map, point::Point};
use anyhow::{Ok, Result};

#[derive(Clone)]
pub struct Traveller {
    starting_point: Option<Point>,
    spt: Vec<Point>,
    pub map: Map,
}

impl Traveller {
    pub fn new(map: Map) -> Result<Self> {
        let spt = Vec::new();

        Ok(Traveller {
            starting_point: None,
            spt,
            map,
        })
    }

    pub fn find_shortest_from_point(&mut self, starting_point: Point) -> Result<usize> {
        // mark started as visited
        self.starting_point = Some(starting_point);
        self.spt.push(self.starting_point.unwrap());
        self.map.map[starting_point.y][starting_point.x].visited = true;

        while !self.spt.is_empty() {
            let current_square = self.spt.remove(0);
            let mut next_squares = self.map.get_possible_moves(current_square);

            for s in &mut next_squares {
                if s.is_target_point {
                    return Ok(current_square.distance);
                } else {
                    self.spt.push(*s);
                };
            }
            self.spt.sort_by(|a, b| a.distance.cmp(&b.distance));
        }

        Ok(10000000000)
    }
}
