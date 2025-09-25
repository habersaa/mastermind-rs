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

#[cfg(test)]
mod tests {
    use crate::pawn::Pawn;

    #[test]
    fn to_index() {
        assert_eq!(Pawn::Red.to_index(), 0);
        assert_eq!(Pawn::Orange.to_index(), 1);
        assert_eq!(Pawn::Yellow.to_index(), 2);
        assert_eq!(Pawn::Green.to_index(), 3);
        assert_eq!(Pawn::Blue.to_index(), 4);
        assert_eq!(Pawn::Black.to_index(), 5);
        assert_eq!(Pawn::Brown.to_index(), 6);
        assert_eq!(Pawn::White.to_index(), 7);
    }

    #[test]
    fn from_index() {
        assert_eq!(Pawn::from_index(0), Pawn::Red);
        assert_eq!(Pawn::from_index(1), Pawn::Orange);
        assert_eq!(Pawn::from_index(2), Pawn::Yellow);
        assert_eq!(Pawn::from_index(3), Pawn::Green);
        assert_eq!(Pawn::from_index(4), Pawn::Blue);
        assert_eq!(Pawn::from_index(5), Pawn::Black);
        assert_eq!(Pawn::from_index(6), Pawn::Brown);
        assert_eq!(Pawn::from_index(7), Pawn::White);

        assert_eq!(Pawn::from_index(8), Pawn::Red);
    }

    #[test]
    fn offset() {
        assert_eq!(Pawn::Red.offset(0), Pawn::Red);
        assert_eq!(Pawn::Red.offset(1), Pawn::Orange);
        assert_eq!(Pawn::Red.offset(2), Pawn::Yellow);
        assert_eq!(Pawn::Red.offset(3), Pawn::Green);
        assert_eq!(Pawn::Red.offset(4), Pawn::Blue);
        assert_eq!(Pawn::Red.offset(5), Pawn::Black);
        assert_eq!(Pawn::Red.offset(6), Pawn::Brown);
        assert_eq!(Pawn::Red.offset(7), Pawn::White);
        assert_eq!(Pawn::Red.offset(9), Pawn::Orange);
        assert_eq!(Pawn::Red.offset(-1), Pawn::White);
        assert_eq!(Pawn::Red.offset(-2), Pawn::Brown);
        assert_eq!(Pawn::Blue.offset(0), Pawn::Blue);
        assert_eq!(Pawn::Blue.offset(1), Pawn::Black);
        assert_eq!(Pawn::Blue.offset(-1), Pawn::Green);
        assert_eq!(Pawn::Blue.offset(5), Pawn::Orange);
    }
}
