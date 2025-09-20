use crate::cell::Cell;
use crate::combination::{CELLS_PER_COMBINATION, Combination};
use crate::game::Game;
use crate::guess::Guess;
use crate::pawn::{PAWN_COLORS_COUNT, Pawn};

pub const TOTAL_COMBINATIONS: usize = PAWN_COLORS_COUNT.pow(CELLS_PER_COMBINATION as u32);

#[derive(Debug)]
pub struct Player {
    possible_solutions: Vec<usize>,
}

impl Player {
    pub fn new() -> Player {
        let solutions = (0..=TOTAL_COMBINATIONS).collect();
        Player {
            possible_solutions: solutions,
        }
    }

    pub fn process_guess(&mut self, guess: &Guess) {
        let mut next_solutions = Vec::new();
        for index in self.possible_solutions.iter() {
            let solution = Self::create_solution(*index);
            let hint = Game::analyze_guess(guess.combination(), &solution);

            if hint == *guess.hint() {
                next_solutions.push(*index);
            }
        }
        self.possible_solutions = next_solutions;
        // println!("Remaining: {}", self.possible_solutions.len());
    }

    pub fn get_guess(&self) -> Combination {
        let mut solution_index = 0;
        let random_solution = self.possible_solutions.len() / 2;
        if random_solution < self.possible_solutions.len() {
            solution_index = self.possible_solutions[random_solution];
        }
        Self::create_solution(solution_index)
    }

    pub fn create_solution(index: usize) -> Combination {
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
