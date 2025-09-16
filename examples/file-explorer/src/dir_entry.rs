use crate::ChangeDir;
use agape::layout::AxisAlignment;
use agape::widgets::{Button, Icon, Text, View, Widget};
use agape::{MessageQueue, hstack};
use std::path::PathBuf;

pub struct DirEntry {
    title: String,
    path: PathBuf,
}

impl DirEntry {
    pub fn new(path: PathBuf, title: &str) -> Self {
        Self {
            path,
            title: title.to_string(),
        }
    }
}

impl View for DirEntry {
    fn update(&mut self, _: &mut MessageQueue) {}

    fn view(&self) -> Box<dyn Widget> {
        let widget = hstack![
            Icon::asset("icons/regular/folder.svg").fixed(24.0, 24.0),
            Text::new(&self.title)
        ]
        .padding(12)
        .cross_axis_alignment(AxisAlignment::Center)
        .spacing(12);

        let message = ChangeDir(self.path.clone());
        let button = Button::new(widget).on_click(move |messages| messages.add(message.clone()));

        Box::new(button)
    }
}
