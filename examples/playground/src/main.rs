use agape::widgets::{HStack, Rect, Text, TextField};
use agape::{App, GlobalId, Widget, hstack};

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    let widget = Rect::new().fixed(100.0, 500.0).background_color(0);
    App::new(widget).run()
}

#[derive(Default, Widget)]
struct Main {
    id: GlobalId,
    #[child]
    widget: HStack,
}

impl Main {
    pub fn new() -> Self {
        let widget = hstack![TextField::new()].fill().align_center();

        Self {
            id: GlobalId::new(),
            widget,
        }
    }
}
