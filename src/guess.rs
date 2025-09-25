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

#[cfg(test)]
mod tests {
    use crate::combination::Combination;
    use crate::guess::{Guess, Hint};
    use crate::pawn::Pawn;

    #[test]
    fn combination() {
        let guess = Guess::new(Combination::new_rainbow());
        assert_eq!(*guess.combination().cell(0).pawn(), Some(Pawn::Red));
        assert_eq!(*guess.combination().cell(1).pawn(), Some(Pawn::Orange));
        assert_eq!(*guess.combination().cell(2).pawn(), Some(Pawn::Yellow));
        assert_eq!(*guess.combination().cell(3).pawn(), Some(Pawn::Green));
        assert_eq!(*guess.combination().cell(4).pawn(), Some(Pawn::Blue));
    }

    #[test]
    fn hint_has_won() {
        let default_hint = Hint::default();
        assert_eq!(default_hint.good_color_and_position, 0);
        assert_eq!(default_hint.good_color_wrong_position, 0);
        assert_eq!(default_hint.has_won(), false);

        assert_eq!(Hint::new(1, 1).has_won(), false);
        assert_eq!(Hint::new(2, 3).has_won(), false);
        assert_eq!(Hint::new(0, 5).has_won(), false);
        assert_eq!(Hint::new(5, 0).has_won(), true);
    }

    #[test]
    fn set_hint() {
        let mut guess = Guess::new(Combination::new_rainbow());
        assert_eq!(guess.hint().good_color_and_position, 0);
        assert_eq!(guess.hint().good_color_wrong_position, 0);

        guess.set_hint(Hint::new(2, 3));
        assert_eq!(guess.hint().good_color_and_position, 2);
        assert_eq!(guess.hint().good_color_wrong_position, 3);
    }
}
