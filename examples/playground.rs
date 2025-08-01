// Playground example, not intended for serious use.
use agape::{App, widgets::*};

fn main() {
    let widget = Container::new(Text::new("Hello world"));
    App::new(widget).run().unwrap()
}
