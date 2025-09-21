use iced::{
    Background, Border, Element, Length, Theme,
    widget::{Container, container, row},
};
use mastermind::{
    combination::{CELLS_PER_COMBINATION, Combination},
    guess::Guess,
};

use crate::{
    EditorType, Message,
    ui::{HINT_ROW_WIDTH, cell, text::hint_text},
};

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

    let mut button = super::button::new_button(text)
        .height(Length::Fill)
        .width(HINT_ROW_WIDTH);
    if combination.is_valid() {
        button = button.on_press(message);
    }

    editor_row = editor_row.push(button);
    Container::new(editor_row)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(palette.background.base.color)),
                border: Border::default()
                    .color(palette.background.base.text)
                    .rounded(5.)
                    .width(1.),
                ..Default::default()
            }
        })
        .into()
}
