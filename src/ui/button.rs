use iced::{
    Length, Renderer, Theme,
    widget::{Button, Text, button},
};

use crate::Message;

pub fn new_button(text: &str) -> Button<'_, Message, Theme, Renderer> {
    button(Text::new(text).center())
        .padding([10, 18])
        .width(Length::Fill)
}
