use agape::{App, MessageQueue, widgets::*};
use tracing::info;

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(Main).run()
}

struct Hover;

#[derive(Default)]
struct Main;

impl View for Main {
    fn update(&mut self, messages: &mut MessageQueue) {
        if messages.has::<Hover>() {
            info!("Hover");
        }
    }
    fn view(&self) -> Box<dyn Widget> {
        let widget = Button::new(Text::new("Hello World!"))
            .on_hover(|messages| messages.add(Hover))
            .on_click(|_| info!("Clicked!"));

        Box::new(widget)
    }
}
