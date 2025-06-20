use helium::{App, widgets::Rect, vstack,Color};

fn main() -> Result<(), helium::Error> {
    let vstack = vstack! {
        Rect::new(200.0, 200.0).color(Color::BLACK),
        Rect::new(200.0, 200.0).color(Color::BLACK),
    }
        .spacing(12)
        .padding(12)
        .color(Color::RED);

    let app = App::new(vstack);
    app.run()
}
