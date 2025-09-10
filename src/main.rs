use iced::{
    Color, Element, Length, Rectangle, Renderer, Theme, mouse,
    widget::{
        Canvas, Container, button,
        canvas::{self, Stroke},
        column, horizontal_rule, row, text,
    },
};
use mastermind::{cell::Cell, combination::Combination, game::Game, pawn::Pawn};

#[derive(Debug)]
struct Mastermind {
    game: Game,
}

impl Default for Mastermind {
    fn default() -> Self {
        let mut game = Game::new();
        game.fill_dummy();
        Self { game }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Start(Combination),
    Play(Combination),
    Reset,
}

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
                Some(Pawn::Brown) => Color::from_rgb(0.5, 0., 0.5),
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

impl Mastermind {
    fn view(&self) -> Element<'_, Message> {
        let mut column = column![];

        for guess_index in 0..10 {
            let mut row = row![];
            for cell_index in 0..5 {
                let cell = Container::new(Canvas::new(self.game.cell(guess_index, cell_index)))
                    .width(40)
                    .height(40)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .padding(5);
                row = row.push(cell);
            }
            row = row.push(
                text(format!(
                    "{} {}",
                    self.game.hint(guess_index).good_color_and_position,
                    self.game.hint(guess_index).good_color_wrong_position
                ))
                .height(Length::Fill)
                .center(),
            );
            column = column.push(row);
        }

        column = column.push(horizontal_rule(1));

        // solution
        let mut row = row![];
        for cell_index in 0..5 {
            let cell = Container::new(Canvas::new(self.game.solution(cell_index)))
                .width(40)
                .height(40)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .padding(5);
            row = row.push(cell);
        }
        column = column.push(row);

        let buttons_row = row![
            button("Reset").on_press(Message::Reset).padding([10, 18]),
            button("Start")
        ];
        column = column.push(buttons_row);
        column = column.padding(5);

        Container::new(column)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Start(combination) => {}
            Message::Play(combination) => {}
            Message::Reset => {
                self.game = Game::new();
            }
        }
    }
}

fn theme(_state: &Mastermind) -> Theme {
    Theme::Dark
}

pub fn main() -> iced::Result {
    iced::application("Mastermind Iced", Mastermind::update, Mastermind::view)
        .theme(theme)
        .run()
}
