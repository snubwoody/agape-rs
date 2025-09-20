use agape::layout::{BlockLayout, EmptyLayout, Layout};
use agape::renderer::Renderer;
use agape::{App, GlobalId, MessageQueue, Widget, widgets::Text};
use tracing::info;

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(Main::new("Hello world")).run()
}

#[derive(Default, Widget)]
struct Main {
    id: GlobalId,
    #[child]
    text: Text,
}

impl Main {
    pub fn new(text: &str) -> Self {
        Self {
            id: GlobalId::new(),
            text: Text::new(text),
        }
    }
}

//
// impl Widget for Main {
//     fn id(&self) -> GlobalId {
//         self.id
//     }
//
//     fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
//         f(&mut self.child);
//         self.child.traverse(f);
//     }
//
//     fn children(&self) -> Vec<&dyn Widget> {
//         vec![&self.child]
//     }
//
//     fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
//         let child_layout = self.child.layout(renderer);
//         let mut layout = BlockLayout::new(child_layout);
//         layout.id = self.id;
//         Box::new(layout)
//     }
//
//     fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
//         self.child.render(renderer, layout);
//     }
// }
