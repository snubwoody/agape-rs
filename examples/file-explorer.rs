#![allow(non_snake_case)]
use agape::widgets::{Text, Widget};
use agape::{App, hstack, vstack};

fn main() -> agape::Result<()> {
    let main = hstack! {
        Sidebar(),
    };

    App::new(main).run()
}

fn Sidebar() -> impl Widget {
    vstack! {
        Text::new("Downloads"),
        Text::new("Documents"),
        Text::new("Music"),
        Text::new("Pictures"),
        Text::new("Videos"),
    }
    .spacing(12)
    .padding(24)
}
