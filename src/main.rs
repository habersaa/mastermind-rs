mod ui;

use iced::{
    Element, Length, Settings, Size, Theme,
    widget::{Container, Text, button, column, horizontal_rule, row},
    window,
};
use mastermind::{
    combination::Combination,
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
    Edit(EditorType, usize, i32),
    Reset,
}

impl Mastermind {
    fn view(&self) -> Element<'_, Message> {
        let mut column = column![].spacing(10);

        for guess_index in 0..MAX_GUESS_COUNT {
            if guess_index == self.game.guess_count() && *self.game.state() == State::Playing {
                column = column.push(ui::edit_combination_view(
                    &self.guess_editor,
                    EditorType::Guess,
                ));
            } else {
                column = column.push(ui::combination_view(self.game.guess(guess_index)));
            }
        }

        match self.game.state() {
            State::WaitForStart => {
                // solution editor
                column = column.push(horizontal_rule(3));
                column = column.push(ui::edit_combination_view(
                    &self.solution_editor,
                    EditorType::Solution,
                ));
            }
            State::Playing => {}
            State::Finished => {
                column = column.push(ui::finished_text(self.game.has_won()));
            }
        }

        let buttons_row = row![
            button(Text::new("Reset").center(),)
                .on_press(Message::Reset)
                .padding([10, 18])
                .width(Length::Fill),
        ];
        column = column.push(buttons_row);
        column = column.padding(5);

        Container::new(column)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .padding(10)
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
            Message::Edit(editor_type, cell_index, delta) => match editor_type {
                EditorType::Guess => {
                    let cell = self.guess_editor.cell(cell_index).next_pawn(delta);
                    self.guess_editor.set_cell(cell_index, cell);
                }
                EditorType::Solution => {
                    let cell = self.solution_editor.cell(cell_index).next_pawn(delta);
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
    let settings = Settings {
        antialiasing: true,
        ..Default::default()
    };
    let window = window::Settings {
        size: Size::new(600., 1000.),
        ..Default::default()
    };

    iced::application("Mastermind Iced", Mastermind::update, Mastermind::view)
        .theme(theme)
        .settings(settings)
        .window(window)
        .run()
}
