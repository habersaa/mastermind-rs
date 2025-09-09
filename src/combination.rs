use crate::{cell::Cell, pawn::Pawn};

pub const CELLS_PER_COMBINATION: usize = 5;

#[derive(Debug, Default, Clone)]
pub struct Combination {
    cells: [Cell; CELLS_PER_COMBINATION],
}

impl Combination {
    pub fn new(cells: [Cell; CELLS_PER_COMBINATION]) -> Combination {
        Combination { cells }
    }

    pub fn new_from_pawns(pawns: [Pawn; CELLS_PER_COMBINATION]) -> Combination {
        Combination {
            cells: std::array::from_fn(|i| Cell::new(pawns[i])),
        }
    }

    pub fn cells(&self) -> &[Cell; CELLS_PER_COMBINATION] {
        &self.cells
    }

    pub fn is_valid(&self) -> bool {
        self.cells.iter().all(|cell| cell.is_valid())
    }
}
