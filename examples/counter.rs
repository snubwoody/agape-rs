use agape::widgets::*;
use agape::{App, hstack};

fn counter(count: i32) -> impl Widget {
    let mut value = count;

    hstack! {
        Button::new(Text::new("Subtract"))
        .on_click(move ||value -= 1),
        Text::new(&format!("{value}")),
        Button::new(Text::new("Add"))
        .on_click(move ||value += 1),
    }
    .fill()
    .align_center()
    .spacing(24)
}

fn main() -> Result<(), agape::Error> {
    App::new(counter(0)).run()
}
