use agape::{App, MessageQueue, hstack, widgets::*};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct ChangeDir(PathBuf);

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    let home = Home::new();
    App::new(home).run()
}

struct Home {
    navbar: Navbar,
    directories: Vec<DirEntry>,
}

impl Home {
    pub fn new() -> Self {
        let home_dir = std::env::home_dir().unwrap();
        let directories = Self::entries(home_dir);
        let navbar = Navbar::new();

        Self {
            directories,
            navbar,
        }
    }

    pub fn entries(dir: PathBuf) -> Vec<DirEntry> {
        let mut directories = vec![];
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let title = entry.file_name().into_string().unwrap();
            directories.push(DirEntry::new(entry.path(), &title));
        }
        directories
    }
}

impl View for Home {
    fn update(&mut self, messages: &mut MessageQueue) {
        // TODO: Check if directory
        if let Some(change_dir) = messages.get::<ChangeDir>() {
            self.directories = Self::entries(change_dir.0.clone());
        }

        self.directories.iter_mut().for_each(|d| d.update(messages));
        self.navbar.update(messages);
    }

    fn view(&self) -> Box<dyn Widget> {
        let mut vstack = VStack::new();
        for entry in &self.directories {
            vstack.append_child(entry.view());
        }
        Box::new(vstack)
    }
}

struct Navbar {
    back: Text,
    forward: Text,
}

impl Navbar {
    pub fn new() -> Self {
        Self {
            back: Text::new("Back"),
            forward: Text::new("Forward"),
        }
    }
}

impl View for Navbar {
    fn view(&self) -> Box<dyn Widget> {
        let back = self.back.clone();
        let forward = self.forward.clone();
        let hstack = hstack! {
            forward,
            back
        }
        .spacing(12);
        Box::new(hstack)
    }
}

struct DirEntry {
    widget: Container<Text>,
}

impl DirEntry {
    pub fn new(_: PathBuf, title: &str) -> Self {
        let widget = Container::new(Text::new(title)).padding(12);
        Self { widget }
    }
}

impl View for DirEntry {
    fn update(&mut self, _: &mut MessageQueue) {}

    fn view(&self) -> Box<dyn Widget> {
        Box::new(self.widget.clone())
    }
}
