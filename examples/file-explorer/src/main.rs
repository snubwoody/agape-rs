#![allow(non_snake_case)]

use agape::{App, Color, Message, State, widgets::*};
use tracing::info;

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    let home = Home::new(["Bank", "Overwatch", "Valorant", "Taxes", "School"]);
    App::new(home).run()
}

struct Home {
    directories: Vec<DirEntry>,
}

impl Home {
    pub fn new(entries: impl IntoIterator<Item = &'static str>) -> Self {
        let mut directories = vec![];
        for entry in entries {
            directories.push(DirEntry::new(entry));
        }

        Self { directories }
    }
}

impl View for Home {
    fn update(&mut self, message: &Message, state: &State) {
        self.directories
            .iter_mut()
            .for_each(|d| d.update(message, state));
    }

    fn view(&self) -> Box<dyn Widget> {
        let mut vstack = VStack::new();
        for entry in &self.directories {
            vstack.append_child(entry.view());
        }
        Box::new(vstack)
    }
}

struct DirEntry {
    widget: Container<Text>,
}

impl DirEntry {
    pub fn new(title: &str) -> Self {
        let widget = Container::new(Text::new(title)).padding(12);
        Self { widget }
    }
}

impl View for DirEntry {
    fn update(&mut self, _: &Message, state: &State) {
        let is_hovered = state.is_hovered(&self.widget.id());
        if is_hovered {
            self.widget = self
                .widget
                .clone()
                .background_color(Color::rgb(230, 230, 230));
        } else {
            self.widget = self.widget.clone().background_color(Color::WHITE);
        }
    }

    fn view(&self) -> Box<dyn Widget> {
        Box::new(self.widget.clone())
    }
}
