#![feature(exclusive_range_pattern)]
#![feature(result_option_inspect)]
#![feature(unwrap_infallible)]
extern crate core;

use std::path::Path;
use std::process::exit;

use itertools;
use itertools::Itertools;

use crate::dsu::DSU;
use crate::maze::{Coord, Dir, GenerationAlgorithm, GrowingTree, Image, Maze};

mod dsu;
mod io;
mod maze;

fn main() {
    let in_0_1 = |x: &f64| *x >= 0. && *x <= 1.;
    let gt_0 = |x: &usize| *x > 0;

    let (width, height) = match io::read_n_and_check(2, gt_0, "Maze dimensions (int, >0: x y):") {
        Some(v) => v.into_iter().collect_tuple().unwrap(),
        None => exit(0),
    };
    let exit_probability = match io::read_n_and_check(1, in_0_1, "Exit probability (float, [0,1]):") {
        Some(v) => v[0],
        None => exit(0),
    };
    let fail_factor = match io::read_n_and_check(1, in_0_1, "Fail factor (float, [0,1]):") {
        Some(v) => v[0],
        None => exit(0),
    };

    let mut maze = Maze::new(height, width);
    GrowingTree { fail_factor }.generate(&mut maze);
    maze.add_exits(exit_probability);

    if width * height < 10_000 {
        let path = Path::new("./_image.jpg");
        Image::new(5).draw(path, &maze);
    };

    let mut dsu = DSU::new(maze.width * maze.height + 1);
    let to_plain = |x, y| -> usize { maze.width * y + x + 1 };
    for y in 0..maze.height {
        for x in 0..maze.width {
            if !maze[Coord { x, y }].walls.contains(Dir::S) {
                dsu.merge(to_plain(x, y), to_plain(x, y + 1))
            }
            if !maze[Coord { x, y }].walls.contains(Dir::E) {
                dsu.merge(to_plain(x, y), to_plain(x + 1, y))
            }
        }
    }
    for x in 0..maze.width {
        if !maze[Coord { x, y: 0 }].walls.contains(Dir::N) {
            dsu.merge(0, to_plain(x, 0))
        }
        if !maze[Coord { x, y: maze.height - 1 }].walls.contains(Dir::S) {
            dsu.merge(0, to_plain(x, maze.height - 1))
        }
    }
    for y in 0..maze.height {
        if !maze[Coord { x: 0, y }].walls.contains(Dir::W) {
            dsu.merge(0, to_plain(0, y))
        }
        if !maze[Coord { x: maze.width - 1, y }].walls.contains(Dir::E) {
            dsu.merge(0, to_plain(maze.width - 1, y))
        }
    }

    loop {
        let (x, y) = match io::read_n(2, "x y:") {
            Some(v) => v.into_iter().collect_tuple().unwrap(),
            None => break,
        };
        match dsu.in_same_sets(0, to_plain(x, y)) {
            None => println!("Index out of bounds"),
            Some(x) => println!("{}", x),
        }
        // let (x1, y1, x2, y2) = match io::read_n(4, "x1 y1 x2 y2:") {
        //     Some(v) => v.into_iter().collect_tuple().unwrap(),
        //     None => break,
        // };
        // match dsu.in_same_sets(to_plain(x1, y1), to_plain(x2, y2)) {
        //     None => println!("Index out of bounds"),
        //     Some(x) => println!("{}", x),
        // }
    }
}
