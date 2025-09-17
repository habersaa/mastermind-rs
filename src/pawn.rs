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
    const ALL: [Pawn; PAWN_COLORS_COUNT] = [
        Pawn::Red,
        Pawn::Orange,
        Pawn::Yellow,
        Pawn::Green,
        Pawn::Blue,
        Pawn::Black,
        Pawn::Brown,
        Pawn::White,
    ];

    pub fn to_index(self) -> usize {
        Self::ALL.iter().position(|&c| c == self).unwrap()
    }

    pub fn from_index(index: usize) -> Self {
        Self::ALL[index % Self::ALL.len()]
    }

    pub fn offset(self, x: i32) -> Self {
        let length = Self::ALL.len() as i32;
        let index = self.to_index() as i32;
        let new_index = (index + x).rem_euclid(length);
        Self::from_index(new_index as usize)
    }
}
