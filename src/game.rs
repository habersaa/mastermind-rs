use crate::combination::{CELLS_PER_COMBINATION, Combination};
use crate::guess::{Guess, Hint};
use crate::pawn::PAWN_COLORS_COUNT;

pub const MAX_GUESS_COUNT: usize = 10;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum State {
    #[default]
    WaitForStart,
    Playing,
    Finished,
}

#[derive(Debug, Default)]
pub struct Game {
    guesses: [Guess; MAX_GUESS_COUNT],
    solution: Combination,
    state: State,
    guess_count: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            guesses: std::array::from_fn(|_| Guess::default()),
            solution: Combination::default(),
            state: State::WaitForStart,
            guess_count: 0,
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn start(&mut self, combination: Combination) {
        match self.state {
            State::WaitForStart => {
                self.solution = combination;
                self.state = State::Playing;
            }
            _ => (),
        }
    }

    pub fn add_guess(&mut self, guess: Combination) {
        if self.state == State::Playing && self.guess_count < MAX_GUESS_COUNT {
            self.guesses[self.guess_count] = Guess::new(guess);

            let hint =
                Self::analyze_guess(self.guesses[self.guess_count].combination(), &self.solution);
            self.guesses[self.guess_count].set_hint(hint);

            self.guess_count += 1;
            if self.has_won() || self.guess_count >= MAX_GUESS_COUNT {
                self.state = State::Finished;
            }
        }
    }

    pub fn last_hint(&self) -> Hint {
        if self.guess_count > 0 {
            return *self.guesses[self.guess_count - 1].hint();
        }
        Hint::default()
    }

    pub fn has_won(&self) -> bool {
        self.guess_count > 0 && self.guesses[self.guess_count - 1].hint().has_won()
    }

    pub fn guess(&self, guess_index: usize) -> &Guess {
        &self.guesses[guess_index]
    }

    pub fn solution(&self) -> &Combination {
        &self.solution
    }

    pub fn guess_count(&self) -> usize {
        self.guess_count
    }

    pub fn analyze_guess(guess: &Combination, solution: &Combination) -> Hint {
        let mut good_color_and_position = 0;
        let mut good_color_wrong_position = 0;

        let mut colors_in_guess = [0u8; PAWN_COLORS_COUNT];
        let mut colors_in_solution = [0u8; PAWN_COLORS_COUNT];

        for i in 0..CELLS_PER_COMBINATION {
            let guess_color = guess.cells()[i].pawn().unwrap();
            let solution_color = solution.cells()[i].pawn().unwrap();

            if guess_color == solution_color {
                good_color_and_position += 1;
            } else {
                colors_in_guess[guess_color.to_index()] += 1;
                colors_in_solution[solution_color.to_index()] += 1;
            }
        }

        for color_index in 0..PAWN_COLORS_COUNT {
            good_color_wrong_position +=
                colors_in_guess[color_index].min(colors_in_solution[color_index]) as usize;
        }

        Hint::new(good_color_and_position, good_color_wrong_position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pawn::Pawn;

    #[test]
    fn run_game() {
        let mut game = Game::new();

        assert_eq!(*game.state(), State::WaitForStart);
        assert_eq!(game.has_won(), false);

        game.start(Combination::new_from_pawns([
            Pawn::Red,
            Pawn::Yellow,
            Pawn::Black,
            Pawn::Yellow,
            Pawn::Blue,
        ]));

        assert_eq!(*game.state(), State::Playing);

        game.add_guess(Combination::new_from_pawns([
            Pawn::White,
            Pawn::White,
            Pawn::White,
            Pawn::White,
            Pawn::White,
        ]));
        assert_eq!(*game.state(), State::Playing);
        assert_eq!(game.has_won(), false);
        assert_eq!(game.last_hint().good_color_and_position, 0);
        assert_eq!(game.last_hint().good_color_wrong_position, 0);

        game.add_guess(Combination::new_from_pawns([
            Pawn::Red,
            Pawn::White,
            Pawn::Yellow,
            Pawn::White,
            Pawn::White,
        ]));
        assert_eq!(*game.state(), State::Playing);
        assert_eq!(game.has_won(), false);
        assert_eq!(game.last_hint().good_color_and_position, 1);
        assert_eq!(game.last_hint().good_color_wrong_position, 1);

        game.add_guess(Combination::new_from_pawns([
            Pawn::Black,
            Pawn::Black,
            Pawn::Black,
            Pawn::Black,
            Pawn::Black,
        ]));
        assert_eq!(*game.state(), State::Playing);
        assert_eq!(game.has_won(), false);
        assert_eq!(game.last_hint().good_color_and_position, 1);
        assert_eq!(game.last_hint().good_color_wrong_position, 0);

        game.add_guess(Combination::new_from_pawns([
            Pawn::Blue,
            Pawn::Red,
            Pawn::Yellow,
            Pawn::Black,
            Pawn::Yellow,
        ]));
        assert_eq!(*game.state(), State::Playing);
        assert_eq!(game.has_won(), false);
        assert_eq!(game.last_hint().good_color_and_position, 0);
        assert_eq!(game.last_hint().good_color_wrong_position, 5);

        game.add_guess(Combination::new_from_pawns([
            Pawn::Red,
            Pawn::Yellow,
            Pawn::Black,
            Pawn::Yellow,
            Pawn::Blue,
        ]));

        assert_eq!(*game.state(), State::Finished);
        assert_eq!(game.has_won(), true);
        assert_eq!(game.last_hint().good_color_and_position, 5);
        assert_eq!(game.last_hint().good_color_wrong_position, 0);
    }
}
