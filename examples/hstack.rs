use agape::{
    App, Color, hstack,
    widgets::{Rect, Text},
};

fn main() {
    let hstack = hstack! {
        Rect::new().fixed(100.0,100.0).background_color(Color::BLACK),
        Rect::new().fixed(100.0,100.0).background_color(Color::BLACK),
    }
    .align_center()
    .spacing(12)
    .padding(12)
    .border_width(2.0)
    .border_color(Color::BLACK)
    .background_color(Color::rgb(210, 210, 210));

    let app = App::new(hstack);
    app.run().unwrap();
}
