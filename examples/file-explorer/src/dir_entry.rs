use crate::ChangeDir;
use agape::layout::AxisAlignment;
use agape::widgets::{Button, HStack, Icon, Text};
use agape::{GlobalId, Widget, hstack};
use std::path::PathBuf;

#[derive(Widget)]
pub struct DirEntry {
    id: GlobalId,
    #[child]
    child: Button<HStack>,
}

impl DirEntry {
    pub fn new(path: PathBuf, title: &str) -> Self {
        let widget = hstack![
            Icon::asset("icons/regular/folder.svg").fixed(24.0, 24.0),
            Text::new(title)
        ]
        .padding(12)
        .cross_axis_alignment(AxisAlignment::Center)
        .spacing(12);

        let message = ChangeDir(path.clone());
        let button = Button::new(widget).on_click(move |messages| messages.add(message.clone()));

        Self {
            id: GlobalId::new(),
            child: button,
        }
    }
}
