use helium::colors::{BLACK, BLUE, RED};
use helium::{App, hstack, widgets::Rect};

fn main() -> Result<(), helium::Error> {
    let hstack = hstack! {
        Rect::new(200.0, 200.0).color(BLACK),
        Rect::new(200.0, 200.0).color(BLACK),
    }
        .spacing(12)
        .padding(12)
        .color(RED);

    let app = App::new(hstack);
    app.run()
}
