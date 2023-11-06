pub use coord::{get_rand_coordinates, Coord};
pub use dir::Dir;
pub use growing_tree::GrowingTree;
pub use image::Image;
pub use maze::Maze;

pub mod cell;
pub mod dir;
pub mod image;
pub mod maze;

pub mod coord;
pub mod growing_tree;

pub trait GenerationAlgorithm {
    fn generate(&self, maze: &mut Maze);
}
