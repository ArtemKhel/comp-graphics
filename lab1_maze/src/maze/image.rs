use std::path::Path;

use image::{GenericImage, ImageBuffer, Rgb, RgbImage, SubImage};

use crate::maze::cell::Cell;
use crate::maze::{Dir, Maze};

pub struct Image {
    cell_size: usize,
    offset: usize,
}

struct Colors;

impl Colors {
    pub const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
    pub const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
}

impl Image {
    pub fn new(cell_size: usize) -> Self {
        assert!(cell_size >= 3);
        Image {
            cell_size,
            offset: cell_size - 1,
        }
    }
    fn draw_cell(&self, subimg: &mut SubImage<&mut RgbImage>, cell: &Cell, south: bool, east: bool) {
        for wall in cell.walls.iter() {
            match wall {
                Dir::N => {
                    for x in 0..self.cell_size as u32 {
                        subimg.put_pixel(x, 0, Colors::BLACK);
                    }
                }
                Dir::S if south => {
                    for x in 0..self.cell_size as u32 {
                        subimg.put_pixel(x, (self.cell_size - 1) as u32, Colors::BLACK);
                    }
                }
                Dir::W => {
                    for y in 0..self.cell_size as u32 {
                        subimg.put_pixel(0, y, Colors::BLACK);
                    }
                }
                Dir::E if east => {
                    for y in 0..self.cell_size as u32 {
                        subimg.put_pixel((self.cell_size - 1) as u32, y, Colors::BLACK);
                    }
                }
                _ => {}
            };
        }
    }
    pub fn draw(&self, path: &Path, maze: &Maze) {
        let mut img: RgbImage = ImageBuffer::from_fn(
            (maze.width * self.offset + 1) as u32,
            (maze.height * self.offset + 1) as u32,
            |_, _| Colors::WHITE,
        );

        maze.cells.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, cell)| {
                let south = y == (maze.height - 1);
                let east = x == (maze.width - 1);
                let mut subimg = img.sub_image(
                    (x * self.offset) as u32,
                    (y * self.offset) as u32,
                    self.cell_size as u32,
                    self.cell_size as u32,
                );
                self.draw_cell(&mut subimg, cell, south, east);
            })
        });

        img.save(path).expect("TODO: panic message");
    }
}

#[test]
fn test() {
    let image = Image::new(5);
    let mut maze = Maze::new(100, 100);
    GrowingTree { fail_factor: 0.001 }.generate(&mut maze);
    let path = Path::new("./_image.jpg");
    image.draw(path, &maze);

    assert!(path.exists())
}
