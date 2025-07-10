use agape::{App, hex, hstack, widgets::*};

fn main() {
    let widget = hstack! {
        Text::new("Email"),
        TextField::new().color(hex!("#ffabbc"))
    }
    .fill()
    .spacing(12)
    .align_center();

    App::new(widget).run().unwrap();
}
