use agape::{App, Color, widgets::Rect};

fn main() -> Result<(), agape::Error> {
    let rect = Rect::new(200.0, 200.0).background_color(Color::BLUE);
    let app = App::new(rect);
    app.run()
}
