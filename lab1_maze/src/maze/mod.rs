pub use coord::{get_rand_coordinates, Coord};
pub use dir::Dir;
pub use growing_tree::GrowingTree;
pub use maze::Maze;

pub mod cell;
pub mod dir;
pub mod maze;

pub mod coord;
pub mod growing_tree;
pub mod image;

pub trait GenerationAlgorithm {
    fn generate(&self, maze: &mut Maze);
}
