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
}
