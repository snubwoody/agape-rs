use helium::{App, widgets::Rect, hstack};
use helium::colors::{BLACK, BLUE};

fn main() -> Result<(), helium::Error> {
    let hstack = hstack! {
        Rect::new(200.0, 200.0).color(BLACK),
        Rect::new(200.0, 200.0).color(BLACK),
    }
        .spacing(12)
        .padding(12);
    
    let app = App::new(hstack);
    app.run()
}
