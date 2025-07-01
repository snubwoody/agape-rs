use dotenv::dotenv;
use helium::{
    App, Color, hstack,
    widgets::{Rect, Text},
};

fn main() -> Result<(), helium::Error> {
    let _ = dotenv();
    env_logger::init();

    let hstack = hstack! {
        Rect::new(200.0, 200.0).color(Color::BLACK),
        Rect::new(200.0, 200.0).color(Color::BLACK),
        Text::new("Hello")
    }
    .align_center()
    .spacing(12)
    .padding(12)
    .color(Color::rgb(210,210,210));

    let app = App::new(hstack);
    app.run()
}
