// Playground example, not intended for serious use.
use agape::{App, widgets::*};

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
        let bytes = include_bytes!("assets/PARTYNEXTDOOR Album Cover.jpg");
        Box::new(Text::new("Hello world"))
    }
}
