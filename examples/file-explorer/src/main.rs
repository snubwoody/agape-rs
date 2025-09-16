mod dir_entry;

use agape::{App, MessageQueue, widgets::*};
use dir_entry::DirEntry;
use std::fs;
use std::path::PathBuf;
use tracing::info;

#[derive(Debug, Clone)]
pub struct ChangeDir(PathBuf);

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    let home = Home::new();
    App::new(home).assets("examples/file-explorer/assets").run()
}

struct Home {
    directories: Vec<DirEntry>,
}

impl Home {
    pub fn new() -> Self {
        let home_dir = std::env::home_dir().unwrap();
        let directories = Self::entries(home_dir);

        Self { directories }
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
        if let Some(path) = messages.get::<ChangeDir>() {
            info!("Changing directory: {:?}", path);
            self.directories = Self::entries(path.0.clone());
        }

        self.directories.iter_mut().for_each(|d| d.update(messages));
    }

    fn view(&self) -> Box<dyn Widget> {
        let mut vstack = VStack::new();
        for entry in &self.directories {
            vstack.append_child(entry.view());
        }
        Box::new(vstack)
    }
}
