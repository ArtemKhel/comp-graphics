pub use coord::{Coord, get_rand_coordinates};
pub use dir::Dir;
pub use growing_tree::GrowingTree;
pub use maze::Maze;

pub mod cell;
pub mod dir;
pub mod maze;

pub mod growing_tree;
pub mod coord;

pub trait GenerationAlgorithm {
    fn generate(&self, maze: &mut Maze);
}
