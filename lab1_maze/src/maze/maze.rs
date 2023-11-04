use std::ops::{Index, IndexMut};

use crate::maze::Coord;
use crate::maze::cell::Cell;
use crate::maze::Dir;

#[derive()]
pub struct Maze {
    pub height: usize,
    pub width: usize,
    pub(crate) cells: Vec<Vec<Cell>>,
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
        let cells = vec![vec![Cell::new(); width]; height];
        // let mut rnd = rand::thread_rng();

        let mut maze = Maze {
            height,
            width,
            cells,
        };

        // let directions = [Dir::N, Dir::S, Dir::W, Dir::E];
        // for _ in 0..(height * width) {
        //     let coord = get_rand_coordinates(height, width);
        //     let dir = directions.choose(&mut rnd).unwrap().clone();
        //     maze_.remove_wall_in_direction(coord, dir);
        // }
        maze
    }
    pub fn remove_wall_in_direction(&mut self, coord: Coord, dir: Dir) -> Option<Coord> {
        let next = self.coord_in_direction(coord, dir);

        if let Some(n) = next {
            self[coord].visited = true;
            self[n].visited = true;
            match dir {
                Dir::N => {
                    self[coord].walls &= !Dir::N;
                    self[n].walls &= !Dir::S;
                }
                Dir::S => {
                    self[coord].walls &= !Dir::S;
                    self[n].walls &= !Dir::N;
                }
                Dir::W => {
                    self[coord].walls &= !Dir::W;
                    self[n].walls &= !Dir::E;
                }
                Dir::E => {
                    self[coord].walls &= !Dir::E;
                    self[n].walls &= !Dir::W;
                }
            }
        }
        next
    }
    pub(crate) fn coord_in_direction(&self, coord: Coord, dir: Dir) -> Option<Coord> {
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
}

// impl std::fmt::Display for Maze {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         self.cells.iter().for_each(|row| {
//             row.iter().for_each(|cell| {
//                 write!(f, "{}", cell).expect("");
//             });
//             write!(f, "\n").expect("");
//         });
//         Ok(())
//     }
// }
