mod dir_entry;

use agape::{App, GlobalId, MessageQueue, Widget, widgets::*};
use dir_entry::DirEntry;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Debug, Clone)]
pub struct ChangeDir(PathBuf);

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    let home = Home::new();
    App::new(home).assets("examples/file-explorer/assets").run()
}

#[derive(Widget)]
#[interactive]
struct Home {
    id: GlobalId,
    #[child]
    child: VStack,
}

// TODO add update method
impl Home {
    pub fn new() -> Self {
        let home_dir = std::env::home_dir().unwrap();
        let directories = Self::entries(home_dir);

        let mut vstack = VStack::new();
        for entry in directories {
            vstack.append_child(Box::new(entry));
        }

        Self {
            id: GlobalId::new(),
            child: vstack,
        }
    }

    pub fn entries(dir: impl AsRef<Path>) -> Vec<DirEntry> {
        let mut directories = vec![];
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let title = entry.file_name().into_string().unwrap();
            directories.push(DirEntry::new(entry.path(), &title));
        }
        directories
    }

    pub fn update(&mut self, messages: &mut MessageQueue) {
        if let Some(change_dir) = messages.get::<ChangeDir>() {
            let path = &change_dir.0;
            self.child.clear();
            let directories = Self::entries(path);
            for entry in directories {
                self.child.append_child(Box::new(entry));
            }
        }
    }
}
