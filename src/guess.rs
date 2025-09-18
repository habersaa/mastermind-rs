use crate::combination::{CELLS_PER_COMBINATION, Combination};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Hint {
    pub good_color_and_position: usize,
    pub good_color_wrong_position: usize,
}

impl Hint {
    pub fn new(good_color_and_position: usize, good_color_wrong_position: usize) -> Hint {
        Hint {
            good_color_and_position,
            good_color_wrong_position,
        }
    }

    pub fn has_won(&self) -> bool {
        self.good_color_and_position == CELLS_PER_COMBINATION
    }
}

#[derive(Debug, Default, Clone)]
pub struct Guess {
    combination: Combination,
    hint: Hint,
}

impl Guess {
    pub fn new(combination: Combination) -> Guess {
        Guess {
            combination,
            hint: Hint::default(),
        }
    }

    pub fn combination(&self) -> &Combination {
        &self.combination
    }

    pub fn set_hint(&mut self, hint: Hint) {
        self.hint = hint;
    }

    pub fn hint(&self) -> &Hint {
        &self.hint
    }
}
