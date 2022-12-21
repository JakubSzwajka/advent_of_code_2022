use anyhow::{bail, Ok, Result};

use super::map::{Map, AIR_SIGN};

pub struct Sand {
    map: Map,
    pub x: usize,
    pub y: usize,
}

impl Sand {
    pub fn new(map: Map) -> Result<Self> {
        Ok(Self {
            x: map.entrance.x,
            y: map.entrance.y,
            map,
        })
    }

    pub fn fall(&mut self) -> Result<()> {
        let mut stop_falling = false;

        while !stop_falling {
            if self.can_fall_down()? && !self.is_endless_void_reached()? {
                self.fall_down()?;
            } else if self.can_fall_down()? && self.is_endless_void_reached()? {
                bail!("Endless void reached");
            } else if self.can_fall_down_left()? {
                self.fall_down_left()?;
            } else if self.can_fall_down_right()? {
                self.fall_down_right()?;
            } else if self.is_entrance_blocked()? {
                bail!("Entrance blocked")
            } else {
                stop_falling = true;
            }
        }
        Ok(())
    }

    fn is_entrance_blocked(&self) -> Result<bool> {
        Ok(self.map.entrance.x == self.x
            && self.map.entrance.y == self.y
            && self.map.is_entrance_blocked())
    }

    fn is_endless_void_reached(&self) -> Result<bool> {
        Ok(self.y == self.map.body.len() - 2)
    }

    fn can_fall_down(&self) -> Result<bool> {
        Ok(self.map.body[self.y + 1][self.x] == AIR_SIGN)
    }

    fn can_fall_down_left(&self) -> Result<bool> {
        Ok(self.map.body[self.y + 1][self.x - 1] == AIR_SIGN)
    }

    fn can_fall_down_right(&self) -> Result<bool> {
        Ok(self.map.body[self.y + 1][self.x + 1] == AIR_SIGN)
    }

    fn fall_down(&mut self) -> Result<()> {
        Ok(self.y += 1)
    }
    fn fall_down_left(&mut self) -> Result<()> {
        self.y += 1;
        self.x -= 1;
        Ok(())
    }
    fn fall_down_right(&mut self) -> Result<()> {
        self.y += 1;
        self.x += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::day14::{map::SAND_SIGN, rock::Rock};

    use super::*;

    #[test]
    fn test_produce_sand() -> Result<()> {
        let mut map = Map::new();
        let rock = Rock::new("490,3 -> 505,3");

        map.add_rock(rock)?;
        map.produce_sand()?;

        assert_eq!(map.body[2][500], SAND_SIGN);
        Ok(())
    }
}
