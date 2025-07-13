use agape::{App, Color, vstack, widgets::*};

fn main() {
    let widget = vstack! {
        Text::new("Email"),
        TextField::new()
        .fixed(100.0, 20.0)
        .background_color(Color::rgb(210,210,210))
        .border_width(2.0)
        .border_color(Color::BLACK)
    }
    .fill()
    .spacing(12)
    .align_center();

    App::new(widget).run().unwrap();
}
