mod ui;

use iced::{
    Element, Length, Settings, Size, Task, Theme,
    widget::{Container, column, horizontal_rule, row, text},
    window,
};
use mastermind::{
    ai::{Player, TOTAL_COMBINATIONS},
    combination::Combination,
    game::{Game, MAX_GUESS_COUNT, State},
};

use crate::ui::AI_ICON;

#[derive(Debug, Copy, Clone, Default)]
struct TestAll {
    current_index: usize,
    victories: usize,
    defeats: usize,
}

#[derive(Debug)]
struct Mastermind {
    game: Game,
    guess_editor: Combination,
    solution_editor: Combination,
    player: Player,
    tests: Option<TestAll>,
}

impl Default for Mastermind {
    fn default() -> Self {
        Self {
            game: Game::new(),
            guess_editor: Combination::new_rainbow(),
            solution_editor: Combination::new_rainbow(),
            player: Player::new(),
            tests: None,
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
    AiStep,
    AiSolve,
    AiTestAll,
    AiTestContinue,
    AiStopTestAll,
}

impl Mastermind {
    fn view(&self) -> Element<'_, Message> {
        let mut column = column![].spacing(10);

        let title_icon = ui::icon(
            ui::TITLE_ICON,
            Size::new(Length::Fill, Length::Fixed(80.0)),
            false,
        );
        column = column.push(row![Container::new(title_icon).center(Length::Fill)]);
        column = column.push(ui::text::intro_text(self.game.state()));

        for guess_index in 0..MAX_GUESS_COUNT {
            if guess_index == self.game.guess_count() && *self.game.state() == State::Playing {
                column = column.push(ui::view::edit_combination_view(
                    &self.guess_editor,
                    EditorType::Guess,
                ));
            } else {
                column = column.push(ui::view::combination_view(self.game.guess(guess_index)));
            }
        }

        let mut buttons_row = row![];
        if *self.game.state() != State::WaitForStart && self.tests.is_none() {
            buttons_row =
                buttons_row.push(ui::button::new_button("Reset").on_press(Message::Reset));
        }

        let mut ai_row = row![].spacing(5);
        if *self.game.state() != State::Finished {
            ai_row = ai_row.push(ui::icon(
                AI_ICON,
                Size::new(Length::Fixed(40.), Length::Fixed(40.)),
                true,
            ));
        }

        if self.tests.is_none() {
            if *self.game.state() == State::WaitForStart {
                ai_row =
                    ai_row.push(ui::button::new_button("Test all").on_press(Message::AiTestAll));
            }
        } else {
            ai_row = ai_row.push(ui::button::new_button("Stop").on_press(Message::AiStopTestAll));
            ai_row = ai_row.push(
                text(format!(
                    "Test:\n{} / {}",
                    self.tests.unwrap().current_index,
                    TOTAL_COMBINATIONS
                ))
                .width(Length::Fill)
                .center(),
            )
        }

        match self.game.state() {
            State::WaitForStart => {
                // solution editor
                column = column.push(horizontal_rule(3));
                column = column.push(ui::view::edit_combination_view(
                    &self.solution_editor,
                    EditorType::Solution,
                ));
            }
            State::Playing => {
                ai_row = ai_row.push(ui::button::new_button("Step").on_press(Message::AiStep));
                ai_row = ai_row.push(ui::button::new_button("Solve").on_press(Message::AiSolve));
                ai_row = ai_row.push(
                    text(format!(
                        "Remaining:\n{} / {}",
                        self.player.remaining_solutions(),
                        TOTAL_COMBINATIONS
                    ))
                    .width(Length::Fill)
                    .center(),
                );
            }
            State::Finished => {
                column = column.push(ui::text::finished_text(self.game.has_won()));
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

    fn update(&mut self, message: Message) -> Task<Message> {
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
            Message::AiStep => {
                let combination = self.player.get_guess();
                self.game.add_guess(combination);
                self.player
                    .process_guess(self.game.guess(self.game.guess_count() - 1));
            }
            Message::AiSolve => {
                while *self.game.state() != State::Finished {
                    let _ = self.update(Message::AiStep);
                }
            }
            Message::AiTestAll => {
                if self.tests.is_none() {
                    self.tests = Some(TestAll::default());
                    return self.update(Message::AiTestContinue);
                }
            }
            Message::AiTestContinue => {
                if self.tests.is_some() {
                    let mut tests = self.tests.unwrap();

                    let _ = self.update(Message::Reset);
                    let _ =
                        self.update(Message::Start(Player::create_solution(tests.current_index)));
                    let _ = self.update(Message::AiSolve);
                    if self.game.has_won() {
                        tests.victories += 1;
                    } else {
                        tests.defeats += 1;
                    }
                    tests.current_index += 1;

                    self.tests = Some(tests);

                    if tests.current_index < TOTAL_COMBINATIONS {
                        return Task::perform(async {}, |_| Message::AiTestContinue);
                    } else {
                        println!(
                            "Finished: W {}, L {}, Total {}",
                            tests.victories, tests.defeats, TOTAL_COMBINATIONS
                        );
                        let _ = self.update(Message::AiStopTestAll);
                    }
                }
            }
            Message::AiStopTestAll => {
                if self.tests.is_some() {
                    self.tests = None;
                }
            }
        }
        Task::none()
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
