mod menu_bar;
use crate::FileInfo;
use agape::state::Context;
use agape::widgets::*;
use agape::{hstack, vstack};
use menu_bar::MenuBar;
use std::fs;

#[derive(Clone)]
pub struct DirState {
    previous_dir: Option<FileInfo>,
    entries: Vec<FileInfo>,
}

impl DirState {
    pub fn new() -> Self {
        let home_dir = std::env::home_dir().unwrap();
        let mut entries = vec![];
        for entry in fs::read_dir(home_dir).unwrap() {
            let entry = entry.unwrap();

            entries.push(FileInfo::from(entry));
        }
        Self {
            previous_dir: Some(entries[0].clone()),
            entries,
        }
    }

    pub fn change_dir(&mut self, entry: FileInfo) {
        // dbg!(&entry);
        if !entry.is_dir {
            return;
        }

        self.entries.clear();
        for entry in fs::read_dir(&entry.path).unwrap() {
            let entry = entry.unwrap();
            let info = FileInfo::from(entry);
            self.entries.push(info.clone());
        }
        self.previous_dir = Some(entry);
    }

    pub fn previous_dir(&mut self) {
        if let Some(previous_dir) = self.previous_dir.clone() {
            self.change_dir(previous_dir);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Page;

impl View for Page {
    type Widget = VStack;

    fn view(&self, ctx: &mut Context) -> Self::Widget {
        ctx.get_or_init(DirState::new);
        vstack![MenuBar.view(ctx), Directories.view(ctx)]
            .fill()
            .spacing(12)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Directories;

impl View for Directories {
    type Widget = VStack;

    fn view(&self, ctx: &mut Context) -> Self::Widget {
        let state = ctx.get_or_init(DirState::new);
        // dbg!(&state.get().entries[0]);
        let mut vstack = VStack::new().spacing(16);
        for entry in state.get().entries {
            let child = DirectoryEntry::new(entry).view(ctx);
            vstack.append_child(child);
        }
        vstack
    }
}

struct DirectoryEntry {
    entry: FileInfo,
}

impl DirectoryEntry {
    pub fn new(entry: FileInfo) -> Self {
        Self { entry }
    }
}

impl View for DirectoryEntry {
    type Widget = Button<HStack>;

    fn view(&self, ctx: &mut Context) -> Self::Widget {
        let entries = ctx.get::<DirState>().clone();
        let entry = self.entry.clone();
        let asset_path = match self.entry.is_dir {
            true => "icons/regular/folder.svg",
            false => "icons/regular/file.svg",
        };
        let icon = Icon::asset(asset_path).fixed(24.0, 24.0);

        Button::new(hstack![icon, Text::new(&self.entry.file_name)].spacing(12))
            .padding_all(4.0)
            .on_click(move |_| entries.update(|entries| entries.change_dir(entry.clone())))
    }
}
