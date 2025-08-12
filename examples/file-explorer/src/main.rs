#![allow(non_snake_case)]
use agape::widgets::{
    LayoutDescription, LayoutType, RenderBox, RenderObject, StateTracker, Text, View, Widget,
};
use agape::{App, Color, GlobalId, IntoColor, Position, Rgba, Size, hstack, vstack};
use agape_renderer::Renderer;
use rand::random;

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
                Dir::new("IMPORTANT!"),
                Dir::new("Bank documents"),
                Dir::new("Work"),
                Dir::new("Taxes"),
                Dir::new("Taxes.docx"),
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

struct Dir {
    id: GlobalId,
    text: Text,
    color: Color<Rgba>,
}

impl Dir {
    pub fn new(name: &str) -> Self {
        Self {
            id: GlobalId::new(),
            text: Text::new(name),
            color: Color::default(),
        }
    }
}

impl Widget for Dir {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn traverse(&self, f: &mut dyn FnMut(&dyn Widget)) {
        f(&self.text);
        self.text.traverse(f);
    }

    fn traverse_mut(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut self.text);
        self.text.traverse_mut(f);
    }

    fn update(&mut self, _state: &StateTracker) {
        let value: u8 = random();
        self.color = value.into_color();
    }

    fn build(&self, renderer: &mut Renderer) -> RenderBox {
        let child = self.text.build(renderer);

        let layout_desc = LayoutDescription {
            layout_type: LayoutType::BlockLayout,
            ..Default::default()
        };

        RenderBox {
            id: self.id,
            children: vec![child],
            position: Position::default(),
            size: Size::default(),
            layout_desc,
            render_object: RenderObject::Rect {
                border: None,
                color: self.color.clone(),
            },
        }
    }
}
