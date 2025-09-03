use agape::{App, widgets::*};

fn main() -> agape::Result<()> {
    App::new(Main).run()
}

struct Main;

impl View for Main {
    fn view(&self) -> Box<dyn Widget> {
        let widget = Container::new(Text::new("Hello World!"));
        Box::new(widget)
    }
}
