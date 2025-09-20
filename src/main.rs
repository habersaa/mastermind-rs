mod ui;

use iced::{
    Element, Length, Settings, Size, Theme,
    widget::{Container, Text, button, column, horizontal_rule, row},
    window,
};
use mastermind::{
    ai::{self, Player},
    combination::Combination,
    game::{Game, MAX_GUESS_COUNT, State},
    pawn::Pawn,
};

#[derive(Debug)]
struct Mastermind {
    game: Game,
    guess_editor: Combination,
    solution_editor: Combination,
    player: Player,
}

impl Default for Mastermind {
    fn default() -> Self {
        Self {
            game: Game::new(),
            guess_editor: Combination::default(),
            solution_editor: Combination::new_from_pawns([
                Pawn::Red,
                Pawn::Orange,
                Pawn::Yellow,
                Pawn::Green,
                Pawn::Blue,
            ]),
            player: Player::new(),
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
    AiOnce,
    AiFinish,
    AiTestAll,
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

        let mut buttons_row = row![];
        buttons_row = buttons_row.push(
            button(Text::new("Reset").center())
                .on_press(Message::Reset)
                .padding([10, 18])
                .width(Length::Fill),
        );
        let mut ai_row = row![];
        ai_row = ai_row.push(
            button(Text::new("Test all").center())
                .on_press(Message::AiTestAll)
                .padding([10, 18])
                .width(Length::Fill),
        );

        match self.game.state() {
            State::WaitForStart => {
                // solution editor
                column = column.push(horizontal_rule(3));
                column = column.push(ui::edit_combination_view(
                    &self.solution_editor,
                    EditorType::Solution,
                ));
            }
            State::Playing => {
                ai_row = ai_row.push(
                    button(Text::new("Play").center())
                        .on_press(Message::AiOnce)
                        .padding([10, 18])
                        .width(Length::Fill),
                );
                ai_row = ai_row.push(
                    button(Text::new("Finish").center())
                        .on_press(Message::AiFinish)
                        .padding([10, 18])
                        .width(Length::Fill),
                );
            }
            State::Finished => {
                column = column.push(ui::finished_text(self.game.has_won()));
            }
        }

        column = column.push(buttons_row);
        column = column.push(ai_row);
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
                if *self.game.state() == State::WaitForStart {
                    self.game.start(combination);
                }
            }
            Message::Play(combination) => {
                if *self.game.state() == State::Playing {
                    self.game.add_guess(combination);
                    self.player
                        .process_guess(self.game.guess(self.game.guess_count() - 1));
                }
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
                self.player = Player::new();
            }
            Message::AiOnce => {
                let combination = self.player.get_guess();
                self.game.add_guess(combination);
                self.player
                    .process_guess(self.game.guess(self.game.guess_count() - 1));
            }
            Message::AiFinish => {
                while *self.game.state() != State::Finished {
                    self.update(Message::AiOnce);
                }
            }
            Message::AiTestAll => {
                let mut victories = 0;
                let mut defeats = 0;
                for i in 0..ai::TOTAL_COMBINATIONS {
                    self.update(Message::Reset);
                    self.update(Message::Start(ai::Player::create_solution(i)));
                    self.update(Message::AiFinish);
                    if self.game.has_won() {
                        victories += 1;
                    } else {
                        defeats += 1;
                    }
                    if i % 100 == 0 {
                        println!("- {}", i);
                    }
                }
                println!(
                    "Finished: W {}, L {}, Total {}",
                    victories,
                    defeats,
                    ai::TOTAL_COMBINATIONS
                );
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
