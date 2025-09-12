use agape::layout::AxisAlignment;
use agape::widgets::{Container, Svg, Text, View, Widget};
use agape::{MessageQueue, hstack};
use std::path::PathBuf;

pub struct DirEntry {
    title: String,
}

impl DirEntry {
    pub fn new(_: PathBuf, title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}

impl View for DirEntry {
    fn update(&mut self, _: &mut MessageQueue) {}

    fn view(&self) -> Box<dyn Widget> {
        let icon = Svg::open("examples/file-explorer/icons/regular/folder.svg")
            .unwrap()
            .fixed(24.0, 24.0);

        let widget = hstack![icon, Text::new(&self.title)]
            .padding(12)
            .cross_axis_alignment(AxisAlignment::Center)
            .spacing(12);
        Box::new(widget)
    }
}
