use crate::cell::Cell;
use crate::combination::{CELLS_PER_COMBINATION, Combination};
use crate::game::Game;
use crate::guess::Guess;
use crate::pawn::{PAWN_COLORS_COUNT, Pawn};

const TOTAL_COMBINATIONS: usize = PAWN_COLORS_COUNT.pow(CELLS_PER_COMBINATION as u32);

#[derive(Debug)]
pub struct Player {
    possible_solutions: [bool; TOTAL_COMBINATIONS],
}

impl Player {
    pub fn new() -> Player {
        Player {
            possible_solutions: [true; TOTAL_COMBINATIONS],
        }
    }

    pub fn process_guess(&mut self, guess: &Guess) {
        let mut count = 0;
        for (index, value) in self.possible_solutions.iter_mut().enumerate() {
            if *value {
                let solution = Self::create_solution(index);
                let hint = Game::analyze_guess(guess.combination(), &solution);

                if hint != *guess.hint() {
                    *value = false;
                } else {
                    count += 1;
                }
            }
        }
        println!("Remaining: {}", count);
    }

    pub fn get_guess(&self) -> Combination {
        for (index, value) in self.possible_solutions.iter().enumerate() {
            if *value {
                return Self::create_solution(index);
            }
        }
        Combination::default()
    }

    fn create_solution(index: usize) -> Combination {
        let mut solution = Combination::default();
        let mut index = index;
        for cell_index in 0..CELLS_PER_COMBINATION {
            let pawn = Pawn::from_index(index % PAWN_COLORS_COUNT);
            solution.set_cell(cell_index, Cell::new(pawn));
            index /= PAWN_COLORS_COUNT;
        }
        solution
    }
}
