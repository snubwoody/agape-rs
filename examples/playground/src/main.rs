use agape::widgets::{HStack, TextField};
use agape::{App, GlobalId, Widget, hstack};

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(Main::new()).run()
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
