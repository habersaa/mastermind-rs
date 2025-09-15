pub mod cell;

use iced::{
    Element, Length,
    widget::{Container, button, row, text},
};
use mastermind::{combination::Combination, guess::Hint};

use crate::{EditorType, Message};

pub fn finished_text(has_won: bool) -> Element<'static, Message> {
    if has_won {
        row![text("Victory !").height(Length::Fill).center()].into()
    } else {
        row![text("Defeat !").height(Length::Fill).center()].into()
    }
}

pub fn hint_text(hint: Hint) -> Element<'static, Message> {
    text(format!(
        "{} {}",
        hint.good_color_and_position, hint.good_color_wrong_position
    ))
    .height(Length::Fill)
    .center()
    .into()
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

    let mut button = button(text).padding([10, 18]).height(Length::Fill);
    if combination.is_valid() {
        button = button.on_press(message);
    }

    editor_row = editor_row.push(button);
    Container::new(editor_row).into()
}
