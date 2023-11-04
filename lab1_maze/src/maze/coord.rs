use rand::Rng;

#[derive(Clone, Copy)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn unpack(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

pub fn get_rand_coordinates(h: usize, w: usize) -> Coord {
    let x = rand::thread_rng().gen_range(0..w);
    let y = rand::thread_rng().gen_range(0..h);
    Coord { x, y }
}
