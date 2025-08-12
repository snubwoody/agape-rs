// Playground example, not intended for serious use.
use agape::{App, Message, widgets::*};

fn main() {
    App::new(RectBox::new()).run().unwrap()
}

#[derive(Debug, Default)]
struct RectBox {
    rect: Rect,
}

impl RectBox {
    pub fn new() -> Self {
        Self {
            rect: Rect::new().fixed(250.0, 250.0),
        }
    }
}

impl View for RectBox {
    fn update(&mut self, _: &Message) {
        // TODO:
        // - store the container
        // - get the layout
        // - check the bounds
    }

    fn view(&self) -> Box<dyn Widget> {
        // TODO: maybe return a reference
        Box::new(self.rect.clone())
    }
}
