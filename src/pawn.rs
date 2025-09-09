#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Pawn {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Black,
    Brown,
    White,
}

pub const PAWN_COLORS_COUNT: usize = 8;

impl Pawn {
    #[inline]
    pub fn index(&self) -> usize {
        self.clone() as u8 as usize
    }
}
