use iced::{
    Element, Length,
    widget::{Container, text},
};
use mastermind::guess::Hint;

use crate::{Message, ui::HINT_ROW_WIDTH};

pub fn finished_text(has_won: bool) -> Element<'static, Message> {
    let text_string = if has_won { "You won !" } else { "You lost !" };
    Container::new(text(text_string).size(24).height(Length::Fill).center()).into()
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
