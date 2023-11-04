use enumflags2::{BitFlags, make_bitflags};
use crate::maze::Dir;

#[derive(Clone, Debug)]
pub struct Cell {
    pub(crate) walls: BitFlags<Dir>,
    pub(crate) visited: bool,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            walls: make_bitflags!(Dir::{N | S | E |W}),
            visited: false,
        }
    }
}

// impl fmt::Display for Cell {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Cell::Empty => write!(f, " "),
//             Cell::Wall => write!(f, "#"),
//             Cell::Start => write!(f, "@"),
//             Cell::Exit => write!(f, "+")
//         }
//     }
// }
