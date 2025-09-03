use agape::{App, Color, hstack, vstack, widgets::*};

fn main() -> agape::Result<()> {
    App::new(Main).run()
}

struct Main;

impl View for Main {
    fn view(&self) -> Box<dyn Widget> {
        let widget = hstack![
            Rect::new()
                .fixed(100.0,100.0)
                .background_color(Color::AMBER);3
        ]
        .fill()
        .spacing(32);

        Box::new(widget)
    }
}
