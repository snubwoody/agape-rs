use agape::{App, Color, widgets::Rect};

fn main() -> Result<(), agape::Error> {
    let rect = Rect::new(200.0, 200.0).color(Color::BLUE);
    let app = App::new(rect);
    app.run()
}
