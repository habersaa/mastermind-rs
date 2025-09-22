use iced::{
    Element, Length,
    widget::{Container, text, tooltip},
};
use mastermind::{game::State, guess::Hint};

use crate::{
    Message,
    ui::{HINT_ROW_WIDTH, tooltip_container},
};

pub fn intro_text(state: &State) -> Element<'static, Message> {
    let text_string = match state {
        State::WaitForStart => "Enter the combination to find !",
        State::Playing => "Enter your guess !",
        State::Finished => "Game over !",
    };
    Container::new(text(text_string).size(24).height(Length::Fill).center()).into()
}

pub fn finished_text(has_won: bool) -> Element<'static, Message> {
    let text_string = if has_won { "You won !" } else { "You lost !" };
    Container::new(text(text_string).size(24).height(Length::Fill).center()).into()
}

pub fn hint_text(hint: &Hint) -> Element<'static, Message> {
    let tooltip_text = format!(
        "Good color and good position: {}\nGood color but bad position: {}",
        hint.good_color_and_position, hint.good_color_wrong_position
    );
    tooltip(
        text(format!(
            "{} {}",
            hint.good_color_and_position, hint.good_color_wrong_position
        ))
        .size(24)
        .height(Length::Fill)
        .width(HINT_ROW_WIDTH)
        .center(),
        tooltip_container(tooltip_text),
        tooltip::Position::Bottom,
    )
    .into()
}
