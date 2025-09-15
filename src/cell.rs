use crate::pawn::Pawn;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Cell {
    pawn: Option<Pawn>,
}

impl Cell {
    pub fn new(pawn: Pawn) -> Cell {
        Cell { pawn: Some(pawn) }
    }

    pub fn pawn(&self) -> &Option<Pawn> {
        &self.pawn
    }

    pub fn is_valid(&self) -> bool {
        match self.pawn {
            Some(_pawn) => true,
            None => false,
        }
    }

    pub fn next_pawn(self) -> Self {
        let pawn = match self.pawn {
            Some(Pawn::Black) => Some(Pawn::Blue),
            Some(Pawn::Blue) => Some(Pawn::Brown),
            Some(Pawn::Brown) => Some(Pawn::Green),
            Some(Pawn::Green) => Some(Pawn::Orange),
            Some(Pawn::Orange) => Some(Pawn::Red),
            Some(Pawn::Red) => Some(Pawn::White),
            Some(Pawn::White) => Some(Pawn::Yellow),
            Some(Pawn::Yellow) => Some(Pawn::Black),
            None => Some(Pawn::Black),
        };
        Self { pawn }
    }
}
