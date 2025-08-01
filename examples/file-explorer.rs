#![allow(non_snake_case)]
use agape::widgets::{Text, Widget};
use agape::{App, hstack, vstack};

fn main() -> agape::Result<()> {
    let main = vstack! {
        hstack!{
            Text::new("Home"),
        }
        .padding(12),
        hstack!{
            Sidebar(),
            vstack!{
                DirEntry("IMPORTANT!"),
                DirEntry("Work"),
                DirEntry("Taxes"),
                DirEntry("Bank documents"),
                DirEntry("Taxes.docx"),
            }
            .spacing(12)
        },
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

fn DirEntry(name: &str) -> impl Widget {
    hstack! {
        Text::new(name),
        Text::new("9/9/2024"),
        Text::new("50 MB"),
    }
    .spacing(32)
}
