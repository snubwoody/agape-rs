use agape::layout::BlockLayout;
use agape::layout::Layout;
use agape::widgets::*;
use agape::{App, GlobalId, hstack};

#[derive(Default)]
struct Counter {
    id: GlobalId,
    count: i32,
}

impl Counter {
    pub fn child(&self) -> impl Widget {
        hstack! {
            Text::new(&format!("Count: {}", self.count))
        }
    }

    pub fn update(&mut self) {}
}

impl Widget for Counter {
    fn layout(&self) -> Box<dyn Layout> {
        let child_layout = self.child().layout();
        let layout = BlockLayout::new(child_layout);
        Box::new(layout)
    }

    fn id(&self) -> GlobalId {
        self.id
    }
}

fn main() -> Result<(), agape::Error> {
    App::new(Counter::default()).run()
}
