#![feature(exclusive_range_pattern)]
extern crate core;

use std::path::Path;
use std::process::exit;

use crate::dsu::DSU;
use crate::image::Image;
use crate::maze::{Coord, Dir, GenerationAlgorithm, GrowingTree, Maze};

mod dsu;
mod maze;
mod image;

fn main() {
    let mut maze = Maze::new(10, 10);
    GrowingTree { fail_factor: 0.1 }.generate(&mut maze);
    let path = Path::new("./_image.svg");

    Image::new(1, 5).create_image(path, &maze);

    let mut dsu = DSU::new(maze.width * maze.height);
    let to_plain = |x, y| -> usize { maze.width * y + x };
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

    for line in std::io::stdin().lines() {
        match line {
            Ok(line) => {
                let coords: Option<Vec<usize>> = line.trim()
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().ok())
                    .collect();

                if let Some(coords) = coords {
                    if coords.len() != 4 { continue; }
                    println!("{}",dsu.in_same_sets(
                        to_plain(coords[0], coords[1]),
                        to_plain(coords[2], coords[3]),
                    ));
                }
            }
            Err(_) => {}
        }
    }
    // loop {
    //     let mut buffer = String::new();
    //     std::io::stdin().read_line(&mut buffer).expect("");
    //
    //     let coords: Option<Vec<usize>> = buffer.trim()
    //         .split_whitespace()
    //         .map(|s| s.parse::<usize>().ok())
    //         .collect();
    //
    //     if let Some(coords) = coords {
    //         if coords.len() != 4 { continue }
    //     }
    // }
}
