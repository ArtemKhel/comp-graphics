use crate::maze::Dir;
use enumflags2::{make_bitflags, BitFlags};

#[derive(Clone, Debug)]
pub struct Cell {
    pub walls: BitFlags<Dir>,
    pub visited: bool,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            walls: make_bitflags!(Dir::{N | S | E |W}),
            visited: false,
        }
    }
}
