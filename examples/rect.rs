use helium::colors::{BLACK, BLUE};
use helium::{App, widgets::Rect};

fn main() -> Result<(), helium::Error> {
    let rect = Rect::new(200.0, 200.0).color(BLUE);
    let app = App::new(rect);
    app.run()
}
