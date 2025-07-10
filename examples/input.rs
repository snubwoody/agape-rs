use agape::{App, hstack, vstack, widgets::*};

fn main() {
    let widget = hstack! {
        Text::new("Email"),
        TextField::new()
    }
    .fill()
    .spacing(12)
    .align_center();

    App::new(widget).run().unwrap();
}
