use crate::view::{RectView, View};
use crate::widgets::Widget;
use agape_core::{Color, GlobalId, Rgba};
use agape_layout::{EmptyLayout, Layout};

#[derive(Default, Clone)]
pub struct TextField {
    id: GlobalId,
    text: String,
    color: Color<Rgba>,
}

impl TextField {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget for TextField {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn view(&self) -> Box<dyn View> {
        let mut view = RectView::default();
        view.set_id(self.id);
        Box::new(view)
    }

    fn layout(&self) -> Box<dyn Layout> {
        let layout = EmptyLayout {
            id: self.id,
            ..Default::default()
        };

        Box::new(layout)
    }
}
