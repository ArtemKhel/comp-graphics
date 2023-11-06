use std::ops::{Index, IndexMut};

use rand::Rng;

use crate::maze::cell::Cell;
use crate::maze::Coord;
use crate::maze::Dir;

#[derive()]
pub struct Maze {
    pub height: usize,
    pub width: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Index<Coord> for Maze {
    type Output = Cell;

    fn index(&self, coord: Coord) -> &Self::Output {
        &self.cells[coord.y][coord.x]
    }
}

impl IndexMut<Coord> for Maze {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        &mut self.cells[coord.y][coord.x]
    }
}

impl Maze {
    pub fn new(height: usize, width: usize) -> Self {
        assert!(height * width > 0, "Both maze dimensions should be >0");
        let cells = vec![vec![Cell::new(); width]; height];
        Maze { height, width, cells }
    }
    pub fn remove_wall_in_direction(&mut self, coord: Coord, dir: Dir) -> Option<Coord> {
        let next = self.coord_in_direction(coord, dir);

        match dir {
            Dir::N => self[coord].walls &= !Dir::N,
            Dir::S => self[coord].walls &= !Dir::S,
            Dir::W => self[coord].walls &= !Dir::W,
            Dir::E => self[coord].walls &= !Dir::E,
        }

        if let Some(n) = next {
            match dir {
                Dir::N => self[n].walls &= !Dir::S,
                Dir::S => self[n].walls &= !Dir::N,
                Dir::W => self[n].walls &= !Dir::E,
                Dir::E => self[n].walls &= !Dir::W,
            }
        }
        next
    }
    pub fn coord_in_direction(&self, coord: Coord, dir: Dir) -> Option<Coord> {
        let (x, y) = coord.unpack();
        let valid = match dir {
            Dir::N if y < 1 => false,
            Dir::S if y >= self.height - 1 => false,
            Dir::W if x < 1 => false,
            Dir::E if x >= self.width - 1 => false,
            _ => true,
        };
        if !valid {
            return None;
        }

        Some(match dir {
            Dir::N => Coord { x, y: y - 1 },
            Dir::S => Coord { x, y: y + 1 },
            Dir::W => Coord { x: x - 1, y },
            Dir::E => Coord { x: x + 1, y },
        })
    }

    pub fn add_exits(&mut self, probability: f64) {
        let mut rnd = rand::thread_rng();
        for x in 0..self.width {
            if rnd.gen_bool(probability) {
                self.remove_wall_in_direction(Coord { x, y: 0 }, Dir::N);
            }
            if rnd.gen_bool(probability) {
                self.remove_wall_in_direction(Coord { x, y: self.height - 1 }, Dir::S);
            }
        }
        for y in 0..self.height {
            if rnd.gen_bool(probability) {
                self.remove_wall_in_direction(Coord { x: 0, y }, Dir::W);
            }
            if rnd.gen_bool(probability) {
                self.remove_wall_in_direction(Coord { x: self.width - 1, y }, Dir::E);
            }
        }
    }
}
