use agape::{App, GlobalId, Widget, widgets::Text};

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(Main::new("Hello world")).run()
}

#[derive(Default, Widget)]
struct Main {
    id: GlobalId,
    #[child]
    text: Text,
}

impl Main {
    pub fn new(text: &str) -> Self {
        Self {
            id: GlobalId::new(),
            text: Text::new(text),
        }
    }
}
