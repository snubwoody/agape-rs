#![allow(non_snake_case)]

use agape::{App, Message, widgets::*};
use tracing::info;

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(Home::new()).run()
}

struct Home {
    directory: DirEntry,
}

impl Home {
    pub fn new() -> Self {
        Self {
            directory: DirEntry::new("Bank"),
        }
    }
}

impl View for Home {
    fn update(&mut self, message: &Message) {
        self.directory.update(message);
    }

    fn view(&self) -> Box<dyn Widget> {
        self.directory.view()
    }
}

struct DirEntry {
    title: String,
}

impl DirEntry {
    pub fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
        }
    }
}

impl View for DirEntry {
    fn update(&mut self, message: &Message) {
        info!("{message:?}")
    }
    fn view(&self) -> Box<dyn Widget> {
        Box::new(Text::new(self.title.as_str()))
    }
}
