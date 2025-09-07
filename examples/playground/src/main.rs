use agape::{App, GlobalId, MessageQueue, widgets::*};
use tracing::info;

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(Main::default()).run()
}

struct Hover;

#[derive(Default)]
struct Main(GlobalId);

impl View for Main {
    fn update(&mut self, messages: &mut MessageQueue) {
        if messages.has::<Hover>() {
            info!("Hover");
        }
    }
    fn view(&self) -> Box<dyn Widget> {
        let mut widget =
            Button::new(Text::new("Hello World!")).on_hover(|messages| messages.add(Hover));

        widget.set_id(self.0);
        Box::new(widget)
    }
}
