use helium::{App};
use helium::widgets::Text;

fn main() -> Result<(), helium::Error> {
    let text = Text::new("Hello, world!");

    let app = App::new(text);
    app.run()
}