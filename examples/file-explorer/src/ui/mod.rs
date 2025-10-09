mod menu_bar;
use crate::FileInfo;
use agape::state::StateCell;
use agape::widgets::*;
use agape::{hstack, vstack};
use menu_bar::MenuBar;
use std::fs;

#[derive(Debug, Clone, Default)]
pub struct Page;

impl StatelessWidget for Page {
    type Widget = VStack;

    fn build(&self) -> Self::Widget {
        vstack![MenuBar.build(), Directories::new().build()]
            .fill()
            .spacing(12)
    }
}

pub struct Directories {
    entries: StateCell<Vec<FileInfo>>,
}

impl Directories {
    pub fn new() -> Self {
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
        dbg!("Click");
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
    type Widget = Button<HStack>;
    fn build(&self) -> Self::Widget {
        let entries = self.entries.clone();
        let entry = self.entry.clone();
        let asset_path = match self.entry.is_dir {
            true => "icons/regular/folder.svg",
            false => "icons/regular/file.svg",
        };
        let icon = Icon::asset(asset_path).fixed(24.0, 24.0);

        Button::new(hstack![icon, Text::new(&self.entry.file_name)].spacing(12))
            .padding(4)
            .on_click(move |_| Self::update_entry(entry.clone(), entries.clone()))
    }
}
