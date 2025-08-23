// Play ground example, not intended for serious use.
use agape::{App, Color, MessageQueue, State, widgets::*};

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
            rect: Rect::new()
                .fixed(250.0, 250.0)
                .background_color(Color::BLACK),
        }
    }
}

impl View for RectBox {
    fn update(&mut self, _: &State, _: &mut MessageQueue) {}

    fn view(&self) -> Box<dyn Widget> {
        // TODO: maybe return a reference
        Box::new(self.rect.clone())
    }
}
