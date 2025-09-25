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

    pub fn next_pawn(self, delta: i32) -> Self {
        match self.pawn {
            Some(pawn) => Cell::new(pawn.offset(delta)),
            None => Cell::new(Pawn::Black),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cell::Cell;
    use crate::pawn::Pawn;

    #[test]
    fn is_valid() {
        assert_eq!(Cell::default().is_valid(), false);
        assert_eq!(Cell::new(Pawn::Green).is_valid(), true);
    }

    #[test]
    fn pawn() {
        assert_eq!(*Cell::default().pawn(), None);
        assert_eq!(*Cell::new(Pawn::Green).pawn(), Some(Pawn::Green));
    }

    #[test]
    fn next_pawn() {
        assert_eq!(*Cell::default().next_pawn(1).pawn(), Some(Pawn::Black));
        assert_eq!(*Cell::default().next_pawn(5).pawn(), Some(Pawn::Black));
        assert_eq!(
            *Cell::new(Pawn::Green).next_pawn(1).pawn(),
            Some(Pawn::Blue)
        );
        assert_eq!(
            *Cell::new(Pawn::Green).next_pawn(-1).pawn(),
            Some(Pawn::Yellow)
        );
    }
}
