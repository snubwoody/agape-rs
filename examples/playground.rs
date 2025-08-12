// Playground example, not intended for serious use.
use agape::{App, Color, widgets::*};

fn main() {
    App::new(RectBox::default()).run().unwrap()
}

#[derive(Debug, Default)]
struct RectBox {
    width: f32,
    height: f32,
}

impl View for RectBox {
    fn update(&mut self) {
        self.width += 1.0;
        self.height += 0.5;
    }

    fn view(&self) -> Box<dyn Widget> {
        Box::new(
            Container::new(
                Rect::new()
                    .fixed(100.0, 200.0)
                    .background_color(Color::BLACK),
            )
            .background_color(Color::RED),
        )
    }
}
