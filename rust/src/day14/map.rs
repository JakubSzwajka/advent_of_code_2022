use super::{
    point::Point,
    rock::{Rock, ROCK_SIGN},
    sand::Sand,
};
use anyhow::{bail, Result};
use core::fmt;

pub const MAP_SIZE_OFFSET: usize = 480;
pub const AIR_SIGN: char = '.';
pub const SAND_SIGN: char = 'O';
const MAP_SIZE: usize = 1000;
const ENTRANCE_SIGN: char = '+';

#[derive(Clone)]
pub struct Map {
    pub body: Vec<Vec<char>>,
    pub entrance: Point,
    lowest_rock_y: usize,
}

impl Map {
    pub fn new() -> Self {
        let mut map = vec![vec![AIR_SIGN; MAP_SIZE]; MAP_SIZE / 2];
        let entrance = Point::new(500, 0);
        map[entrance.y][entrance.x] = ENTRANCE_SIGN;
        Self {
            body: map,
            entrance: entrance,
            lowest_rock_y: 0,
        }
    }

    pub fn add_rock(&mut self, rock: Rock) -> Result<()> {
        for p in rock.get_rock_coordinates()? {
            self.body[p.y][p.x] = ROCK_SIGN;
            self.update_lowest_rock(&p)?;
        }
        Ok(())
    }

    pub fn produce_sand(&mut self) -> Result<()> {
        let mut sand = Sand::new(self.clone())?;
        match sand.fall() {
            Ok(()) => self.add_sand(&sand),
            Err(e) => bail!(e),
        }
    }

    pub fn add_sand(&mut self, sand: &Sand) -> Result<()> {
        self.body[sand.y][sand.x] = SAND_SIGN;
        Ok(())
    }

    fn update_lowest_rock(&mut self, rock: &Point) -> Result<()> {
        if self.lowest_rock_y < rock.y {
            self.lowest_rock_y = rock.y;
        }
        Ok(())
    }

    pub fn add_floor(&mut self) -> Result<()> {
        for p in &mut self.body[self.lowest_rock_y + 2] {
            *p = ROCK_SIGN;
        }
        Ok(())
    }

    pub fn is_entrance_blocked(&self) -> bool {
        self.body[self.entrance.y][self.entrance.x] == SAND_SIGN
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "")?;
        for i in &self.body[0..30] {
            let row = i[MAP_SIZE_OFFSET..MAP_SIZE_OFFSET + 50]
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            writeln!(f, "{}", row)?;
        }
        writeln!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_rock() -> Result<()> {
        let rock = Rock::new("498,4 -> 498,5 -> 499,5");
        let mut map = Map::new();

        map.add_rock(rock)?;

        assert_eq!(map.body[4][498], ROCK_SIGN);
        assert_eq!(map.body[5][498], ROCK_SIGN);
        assert_eq!(map.body[5][499], ROCK_SIGN);
        Ok(())
    }
}
