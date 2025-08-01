#![allow(non_snake_case)]
use agape::widgets::{RenderBox, Text, Widget};
use agape::{App, GlobalId, hstack, vstack};

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

struct Dir {
    id: GlobalId,
    name: String,
}

impl Dir {
    pub fn new(name: &str) -> Self {
        Self {
            id: GlobalId::new(),
            name: name.to_string(),
        }
    }
}

impl Widget for Dir {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn build(&self) -> RenderBox {
        Text::new(&self.name).build()
    }
}
