use agape::MessageQueue;
use agape::widgets::{Container, Svg, Text, View, Widget};
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
        Svg::open("phosphor-icons/regular");
        let widget = Container::new(Text::new(&self.title)).padding(12);
        Box::new(widget)
    }
}
