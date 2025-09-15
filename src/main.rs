mod ui;

use std::cmp::min;

use iced::{
    Element, Length, Theme,
    widget::{Container, button, column, horizontal_rule, row},
};
use mastermind::{
    combination::{CELLS_PER_COMBINATION, Combination},
    game::{Game, MAX_GUESS_COUNT, State},
};

#[derive(Debug)]
struct Mastermind {
    game: Game,
    guess_editor: Combination,
    solution_editor: Combination,
}

impl Default for Mastermind {
    fn default() -> Self {
        let mut game = Game::new();
        game.fill_dummy();
        Self {
            game: game,
            guess_editor: Combination::default(),
            solution_editor: Combination::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum EditorType {
    Solution,
    Guess,
}

#[derive(Debug, Clone)]
enum Message {
    Start(Combination),
    Play(Combination),
    Edit(EditorType, usize),
    Reset,
}

impl Mastermind {
    fn view(&self) -> Element<'_, Message> {
        let mut column = column![];

        for guess_index in 0..min(MAX_GUESS_COUNT, self.game.guess_count()) {
            let mut row = row![];
            for cell_index in 0..CELLS_PER_COMBINATION {
                row = row.push(ui::cell::cell_view(self.game.cell(guess_index, cell_index)));
            }
            row = row.push(ui::hint_text(self.game.hint(guess_index)));
            column = column.push(row);
        }

        match self.game.state() {
            // editor
            State::WaitForStart => {
                column = column.push(ui::edit_combination_view(
                    &self.solution_editor,
                    EditorType::Solution,
                ));
            }
            State::Playing => {
                column = column.push(ui::edit_combination_view(
                    &self.guess_editor,
                    EditorType::Guess,
                ));

                // solution
                // column = column.push(horizontal_rule(1));
                // column = column.push(ui::edit_combination_view(
                //     &self.game.solution(),
                //     EditorType::Solution,
                // ));
            }
            State::Finished => {
                column = column.push(ui::finished_text(self.game.has_won()));
            }
        }

        let buttons_row = row![button("Reset").on_press(Message::Reset).padding([10, 18]),];
        column = column.push(buttons_row);
        column = column.padding(5);

        Container::new(column)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Start(combination) => {
                self.game.start(combination);
            }
            Message::Play(combination) => {
                self.game.add_guess(combination);
            }
            Message::Edit(editor_type, cell_index) => match editor_type {
                EditorType::Guess => {
                    let cell = self.guess_editor.cell(cell_index).next_pawn();
                    self.guess_editor.set_cell(cell_index, cell);
                }
                EditorType::Solution => {
                    let cell = self.solution_editor.cell(cell_index).next_pawn();
                    self.solution_editor.set_cell(cell_index, cell);
                }
            },
            Message::Reset => {
                self.game = Game::new();
            }
        }
    }
}

fn theme(_state: &Mastermind) -> Theme {
    Theme::Dark
}

pub fn main() -> iced::Result {
    iced::application("Mastermind Iced", Mastermind::update, Mastermind::view)
        .theme(theme)
        .run()
}
