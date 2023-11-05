use std::path::Path;

use svg::{Document, Node};
use svg::node::element::path::{Command, Data};
use svg::node::element::Path as SVGPath;
use svg::node::element::path::Position::{Absolute, Relative};

use crate::maze::{Dir, Maze};
use crate::maze::cell::Cell;
use crate::maze::coord::Coord;

pub struct Image {
    line_width: usize,
    cell_size: usize,
}

impl Image {
    pub fn new(line_width: usize, cell_size: usize) -> Self {
        Image {
            line_width,
            cell_size,
        }
    }
    fn draw_cell(&self, coord: Coord, cell: &Cell, data: &mut Data, south: bool, east: bool) {
        let up_left = (coord.x * self.cell_size, coord.y * self.cell_size);
        let down_right = (
            (coord.x + 1) * self.cell_size,
            (coord.y + 1) * self.cell_size,
        );
        for wall in cell.walls.iter() {
            match wall {
                Dir::N => {
                    data.append(Command::Move(Absolute, up_left.into()));
                    data.append(Command::HorizontalLine(Relative, self.cell_size.into()));
                }
                Dir::S if south => {
                    data.append(Command::Move(Absolute, down_right.into()));
                    data.append(Command::HorizontalLine(
                        Relative,
                        (-(self.cell_size as i8)).into(),
                    ));
                }
                Dir::W => {
                    data.append(Command::Move(Absolute, up_left.into()));
                    data.append(Command::VerticalLine(Relative, self.cell_size.into()));
                }
                Dir::E if east => {
                    data.append(Command::Move(Absolute, down_right.into()));
                    data.append(Command::VerticalLine(
                        Relative,
                        (-(self.cell_size as i8)).into(),
                    ));
                }
                _ => {}
            };
        }
    }

    fn create_data(&self, maze: &Maze) -> Data {
        let mut data = Data::new();
        maze.cells.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, cell)| {
                let coord = Coord { x, y };
                let south = y == (maze.height - 1);
                let east = x == (maze.width - 1);
                self.draw_cell(coord, cell, &mut data, south, east);
            })
        });
        data
    }
    pub fn draw(&self, path: &Path, maze: &Maze) {
        let mut document = Document::new().set(
            "viewBox",
            (
                0,
                0,
                maze.width * self.cell_size,
                maze.height * self.cell_size,
            ),
        );
        let data = self.create_data(&maze);
        let svg_path = SVGPath::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", self.line_width)
            .set("d", data);
        document.append(svg_path);

        svg::save(path, &document).unwrap();
    }
}