pub mod button;
pub mod cell;
pub mod text;
pub mod view;

use iced::widget::svg::Status;
use iced::widget::{Container, Svg, container, svg};
use iced::{Background, Border, Element, Length, Size, Theme};

use crate::Message;

const HINT_ROW_WIDTH: Length = Length::Fixed(150.);

pub const TITLE_ICON: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/mastermind.svg"
));
pub const AI_ICON: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/ai.svg"));

pub fn icon(
    icon_data: &'static [u8],
    size: Size<Length>,
    colorize: bool,
) -> Element<'static, Message> {
    let handle = svg::Handle::from_memory(icon_data);

    let mut icon = Svg::new(handle).width(size.width).height(size.height);
    if colorize {
        icon = icon.style(|theme: &Theme, _status: Status| svg::Style {
            color: Some(theme.palette().text),
        })
    }
    icon.into()
}

pub fn tooltip_container(tooltip_text: String) -> Element<'static, Message> {
    Container::new(iced::widget::text(tooltip_text))
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
        .padding(5)
        .into()
}
