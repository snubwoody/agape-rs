use agape::widgets::{HStack, Rect, Text, TextField};
use agape::{App, GlobalId, Widget, hstack};

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    let widget = Rect::new().fixed(100.0, 500.0).background_color(0);
    App::new(widget).run()
}
