mod dir_entry;

use agape::state::StateCell;
use agape::{App, GlobalId, MessageQueue, Widget, vstack, widgets::*};
use dir_entry::DirEntry;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ChangeDir(PathBuf);

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    let home = Home::new();
    App::new(Directories::new())
        .assets("examples/file-explorer/assets")
        .run()
}

#[derive(Debug, Clone)]
struct FileInfo {
    file_name: String,
    is_dir: bool,
    path: PathBuf,
}

impl From<fs::DirEntry> for FileInfo {
    fn from(entry: fs::DirEntry) -> Self {
        let file_name = entry.file_name().to_str().unwrap().to_string();
        let is_dir = entry.file_type().unwrap().is_dir();
        let path = entry.path();
        Self {
            file_name,
            is_dir,
            path,
        }
    }
}
struct Directories {
    entries: StateCell<Vec<FileInfo>>,
}

impl Directories {
    fn new() -> Self {
        let home_dir = std::env::home_dir().unwrap();
        let mut entries = vec![];
        for entry in fs::read_dir(home_dir).unwrap() {
            let entry = entry.unwrap();

            entries.push(FileInfo::from(entry));
        }

        Self {
            entries: StateCell::new(entries),
        }
    }
}

impl StatelessWidget for Directories {
    type Widget = VStack;

    fn build(&self) -> Self::Widget {
        let mut vstack = VStack::new().spacing(16);
        for entry in self.entries.get() {
            let child = DirectoryEntry::new(entry, self.entries.clone()).build();
            vstack.append_child(child);
        }
        vstack
    }
}

struct DirectoryEntry {
    entry: FileInfo,
    entries: StateCell<Vec<FileInfo>>,
}

impl DirectoryEntry {
    pub fn new(entry: FileInfo, entries: StateCell<Vec<FileInfo>>) -> Self {
        Self { entry, entries }
    }

    fn update_entry(entry: FileInfo, entries: StateCell<Vec<FileInfo>>) {
        if !entry.is_dir {
            return;
        }

        entries.update(|entries| entries.clear());
        for entry in fs::read_dir(entry.path).unwrap() {
            let entry = entry.unwrap();
            let info = FileInfo::from(entry);
            entries.update(move |entries| entries.push(info.clone()));
        }
    }
}

impl StatelessWidget for DirectoryEntry {
    type Widget = Button<Text>;
    fn build(&self) -> Self::Widget {
        let entries = self.entries.clone();
        let entry = self.entry.clone();
        Button::text(&self.entry.file_name)
            .padding(4)
            .on_click(move |_| Self::update_entry(entry.clone(), entries.clone()))
    }
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
            vstack.append_child(entry);
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
                self.child.append_child(entry);
            }
        }
    }
}
