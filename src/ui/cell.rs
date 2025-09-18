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

        let radius = bounds.width.min(bounds.height) / 2. - 4.;
        let circle = canvas::Path::circle(frame.center(), radius);

        if self.is_valid() {
            let color = match self.pawn() {
                Some(Pawn::Black) => Color::from_rgb(0.2, 0.2, 0.2), // gris anthracite
                Some(Pawn::Blue) => Color::from_rgb(0.25, 0.45, 0.85), // bleu doux
                Some(Pawn::Brown) => Color::from_rgb(0.55, 0.40, 0.28), // brun chaleureux
                Some(Pawn::Green) => Color::from_rgb(0.30, 0.65, 0.35), // vert tendre
                Some(Pawn::Orange) => Color::from_rgb(0.95, 0.55, 0.25), // orange doux
                Some(Pawn::Red) => Color::from_rgb(0.85, 0.25, 0.30), // rouge brique
                Some(Pawn::White) => Color::from_rgb(0.95, 0.95, 0.95), // blanc cassé
                Some(Pawn::Yellow) => Color::from_rgb(0.95, 0.90, 0.35), // jaune moutarde
                None => Color::from_rgb(0.6, 0.6, 0.6),              // gris neutre pour “vide”
            };
            frame.fill(&circle, color);
            frame.stroke(
                &circle,
                Stroke::default()
                    .with_color(Color::from_rgb(color.r * 1.2, color.g * 1.2, color.b * 1.2))
                    .with_width(3.),
            );
        } else {
            frame.stroke(
                &circle,
                Stroke::default().with_color(Color::from_rgb(0.95, 0.95, 0.95).scale_alpha(0.01)),
            );
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
        .on_press(Message::Edit(editor_type, cell_index, 1))
        .on_right_press(Message::Edit(editor_type, cell_index, -1))
        .into()
}
