use helium::colors::{BLACK, RED};
use helium::{App, widgets::Rect, vstack};

fn main() -> Result<(), helium::Error> {
    let vstack = vstack! {
        Rect::new(200.0, 200.0).color(BLACK),
        Rect::new(200.0, 200.0).color(BLACK),
    }
        .spacing(12)
        .padding(12)
        .color(RED);

    let app = App::new(vstack);
    app.run()
}
