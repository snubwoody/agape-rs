#![allow(non_snake_case)]
use agape::{App, hstack, vstack, widgets::*};

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(Home).run()
}

struct Home;

impl View for Home {
    fn view(&self) -> Box<dyn Widget> {
        let main = vstack! {
        hstack!{
            Text::new("Home"),
        }
        .padding(12),
        hstack!{
            Sidebar(),
            vstack!{
                Text::new("IMPORTANT!"),
                Text::new("Bank documents"),
                Text::new("Work"),
                Text::new("Taxes"),
                Text::new("Taxes.docx"),
            }
            .spacing(12)
        },
        };
        Box::new(main)
    }
}

fn Sidebar() -> impl Widget {
    vstack! {
        QuickAccess(),
        Drives()
    }
}

fn QuickAccess() -> impl Widget {
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

fn Drives() -> impl Widget {
    vstack! {
        Text::new("This PC"),
    }
}
