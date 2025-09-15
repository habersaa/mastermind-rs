use iced::{
    Color, Element, Length, Rectangle, Renderer, Theme, mouse,
    widget::{
        Canvas, Container,
        canvas::{self, Stroke},
        mouse_area,
    },
};
use mastermind::{cell::Cell, combination::Combination, pawn::Pawn};

use crate::{EditorType, Message};

impl canvas::Program<Message> for Cell {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let radius = bounds.width.min(bounds.height) / 2. - 2.;
        let circle = canvas::Path::circle(frame.center(), radius);

        if self.is_valid() {
            let color = match self.pawn() {
                Some(Pawn::Black) => Color::from_rgb(0., 0., 0.),
                Some(Pawn::Blue) => Color::from_rgb(0., 0., 1.),
                Some(Pawn::Brown) => Color::from_rgb(0.28, 0.18, 0.11),
                Some(Pawn::Green) => Color::from_rgb(0., 1., 0.),
                Some(Pawn::Orange) => Color::from_rgb(1., 0.5, 0.),
                Some(Pawn::Red) => Color::from_rgb(1., 0., 0.),
                Some(Pawn::White) => Color::from_rgb(1., 1., 1.),
                Some(Pawn::Yellow) => Color::from_rgb(1., 1., 0.),
                None => Color::BLACK,
            };
            frame.fill(&circle, color);
        } else {
            frame.stroke(&circle, Stroke::default());
        }

        vec![frame.into_geometry()]
    }
}

pub fn cell_view(cell: Cell) -> Element<'static, Message> {
    Container::new(Canvas::new(cell))
        .width(40)
        .height(40)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(5)
        .into()
}

pub fn edit_cell_view(
    cell_index: usize,
    combination: &Combination,
    editor_type: EditorType,
) -> Element<'static, Message> {
    let cell = combination.cell(cell_index);

    mouse_area(cell_view(cell))
        .on_press(Message::Edit(editor_type, cell_index))
        .into()
}
