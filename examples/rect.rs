use agape::{App, Color, widgets::Rect};

fn main() -> Result<(), agape::Error> {
    let rect = Rect::new()
        .fixed(500.0, 500.0)
        .background_color(Color::BLUE);

    App::new(rect).run()
}
