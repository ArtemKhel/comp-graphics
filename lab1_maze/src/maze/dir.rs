use enumflags2::bitflags;
#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Dir {
    N = 0b0001,
    S = 0b0010,
    E = 0b0100,
    W = 0b1000,
}
