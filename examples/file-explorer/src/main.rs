use agape::App;
use std::fs;
use std::path::PathBuf;
mod ui;
use crate::ui::Page;

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(Page).assets("examples/file-explorer/assets").run()
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
