#![allow(non_snake_case)]
use agape::widgets::{RenderBox, Text, Widget};
use agape::{App, GlobalId, hstack, vstack};
use agape_renderer::Renderer;

fn main() -> agape::Result<()> {
    let main = vstack! {
        hstack!{
            Text::new("Home"),
        }
        .padding(12),
        hstack!{
            Sidebar(),
            vstack!{
                Dir::new("IMPORTANT!"),
                Dir::new("Bank documents"),
                Dir::new("Work"),
                Dir::new("Taxes"),
                Dir::new("Taxes.docx"),
            }
            .spacing(12)
        },
    };

    App::new(main).run()
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

    fn build(&self, renderer: &mut Renderer) -> RenderBox {
        Text::new(&self.name).build(renderer)
    }
}
