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

    pub fn new_rainbow() -> Combination {
        Self::new_from_pawns([
            Pawn::Red,
            Pawn::Orange,
            Pawn::Yellow,
            Pawn::Green,
            Pawn::Blue,
        ])
    }

    pub fn cells(&self) -> &[Cell; CELLS_PER_COMBINATION] {
        &self.cells
    }

    pub fn cell(&self, cell_index: usize) -> Cell {
        self.cells[cell_index].clone()
    }

    pub fn set_cell(&mut self, cell_index: usize, cell: Cell) {
        self.cells[cell_index] = cell;
    }

    pub fn is_valid(&self) -> bool {
        self.cells.iter().all(|cell| cell.is_valid())
    }
}

#[cfg(test)]
mod tests {
    use crate::cell::Cell;
    use crate::combination::Combination;
    use crate::pawn::Pawn;

    #[test]
    fn set_cell() {
        let mut c = Combination::new_rainbow();
        assert_eq!(*c.cell(0).pawn(), Some(Pawn::Red));
        c.set_cell(0, Cell::new(Pawn::White));
        assert_eq!(*c.cell(0).pawn(), Some(Pawn::White));
    }

    #[test]
    fn is_valid() {
        assert_eq!(Combination::default().is_valid(), false);

        assert_eq!(
            Combination::new([
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default()
            ])
            .is_valid(),
            false
        );

        assert_eq!(
            Combination::new([
                Cell::new(Pawn::Black),
                Cell::new(Pawn::White),
                Cell::new(Pawn::Yellow),
                Cell::new(Pawn::Green),
                Cell::default()
            ])
            .is_valid(),
            false
        );

        assert_eq!(
            Combination::new_from_pawns([
                Pawn::Black,
                Pawn::White,
                Pawn::Yellow,
                Pawn::Green,
                Pawn::Red,
            ])
            .is_valid(),
            true
        );

        assert_eq!(Combination::new_rainbow().is_valid(), true);
    }
}
