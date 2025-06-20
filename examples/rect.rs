use helium::{App, widgets::Rect};
use helium::colors::{BLACK, BLUE};

fn main() -> Result<(), helium::Error> {
    let rect = Rect::new(200.0, 200.0).color(BLUE);
    let app = App::new(rect);
    app.run()
}
