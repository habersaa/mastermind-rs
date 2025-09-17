pub mod cell;

use iced::{
    Background, Border, Color, Element, Length, Theme,
    widget::{Container, Text, button, container, row, text},
};
use mastermind::{
    combination::{CELLS_PER_COMBINATION, Combination},
    guess::{Guess, Hint},
};

use crate::{EditorType, Message};

const HINT_ROW_WIDTH: Length = Length::Fixed(150.);

pub fn finished_text(has_won: bool) -> Element<'static, Message> {
    if has_won {
        Container::new(text("You won !").height(Length::Fill).center()).into()
    } else {
        Container::new(text("You lost !").height(Length::Fill).center()).into()
    }
}

pub fn hint_text(hint: &Hint) -> Element<'static, Message> {
    text(format!(
        "{} {}",
        hint.good_color_and_position, hint.good_color_wrong_position
    ))
    .size(24)
    .height(Length::Fill)
    .width(HINT_ROW_WIDTH)
    .center()
    .into()
}

pub fn combination_view(guess: &Guess) -> Element<'static, Message> {
    let mut row = row![];
    for cell_index in 0..CELLS_PER_COMBINATION {
        row = row.push(cell::cell_view(guess.combination().cell(cell_index)));
    }
    row = row.push(hint_text(guess.hint()));
    row.into()
}

pub fn edit_combination_view(
    combination: &Combination,
    editor_type: EditorType,
) -> Element<'_, Message> {
    let mut editor_row = row![];

    for cell_index in 0..5 {
        let cell = cell::edit_cell_view(cell_index, combination, editor_type);
        editor_row = editor_row.push(cell);
    }

    let (text, message) = match editor_type {
        EditorType::Guess => ("Add", Message::Play(combination.clone())),
        EditorType::Solution => ("Start", Message::Start(combination.clone())),
    };

    let mut button = button(Text::new(text).center())
        .padding([10, 18])
        .height(Length::Fill)
        .width(HINT_ROW_WIDTH);
    if combination.is_valid() {
        button = button.on_press(message);
    }

    editor_row = editor_row.push(button);
    Container::new(editor_row)
        .style(|_theme: &Theme| container::Style {
            background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.15))),
            border: Border::default()
                .color(Color::from_rgb(0.2, 0.2, 0.2))
                .rounded(5.)
                .width(1.),
            ..Default::default()
        })
        .into()
}
