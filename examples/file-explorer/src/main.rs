use agape::{App, Color, Message, State, widgets::*};
use std::fs;

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    let home = Home::new();
    App::new(home).run()
}

struct Home {
    directories: Vec<DirEntry>,
}

impl Home {
    pub fn new() -> Self {
        let home_dir = std::env::home_dir().unwrap();
        let mut directories = vec![];
        for entry in fs::read_dir(home_dir).unwrap() {
            let title = entry.unwrap().file_name().into_string().unwrap();
            directories.push(DirEntry::new(&title));
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
