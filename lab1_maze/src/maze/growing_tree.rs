use rand::seq::SliceRandom;
use rand::Rng;

use crate::maze::get_rand_coordinates;
use crate::maze::maze::Maze;
use crate::maze::Dir;
use crate::maze::GenerationAlgorithm;

pub struct GrowingTree {
    pub fail_factor: f64,
}

impl GenerationAlgorithm for GrowingTree {
    fn generate(&self, maze: &mut Maze) {
        let mut directions = [Dir::N, Dir::S, Dir::W, Dir::E];
        let mut cells = vec![get_rand_coordinates(maze.height, maze.width)];

        while !cells.is_empty() {
            let mut index = Some(cells.len() - 1);
            let coord = cells[index.unwrap()];

            directions.shuffle(&mut rand::thread_rng());
            for dir in directions {
                let next = match maze.coord_in_direction(coord, dir) {
                    Some(c) => c,
                    None => continue,
                };

                if maze[next].visited {
                    continue;
                }

                let next = if !rand::thread_rng().gen_bool(self.fail_factor) {
                    maze.remove_wall_in_direction(coord, dir)
                } else {
                    maze.coord_in_direction(coord, dir)
                };

                maze[coord].visited = true;
                if let Some(next) = next {
                    maze[next].visited = true;
                    cells.push(next);
                    index = None;
                    break;
                };
            }
            if let Some(i) = index {
                cells.remove(i);
            }
        }
    }
}
