use crate::impl_style;
use crate::style::BoxStyle;
use crate::view::{RectView, View};
use crate::widgets::{Text, Widget};
use agape_core::GlobalId;
use agape_layout::{BlockLayout, Layout};
use winit::event::KeyEvent;

#[derive(Default, Clone)]
pub struct TextField {
    id: GlobalId,
    pub text: Text,
    pub style: BoxStyle,
}

impl TextField {
    pub fn new() -> Self {
        Self::default()
    }

    impl_style!();
}

impl Widget for TextField {
    fn key_input(&mut self, event: &KeyEvent) {
        if let Some(text) = &event.text {
            self.text.text.push_str(text);
        }
    }

    fn id(&self) -> GlobalId {
        self.id
    }

    fn view(&self) -> Box<dyn View> {
        let view = RectView {
            id: self.id,
            color: self.style.background_color.clone(),
            border: self.style.border.clone(),
            ..Default::default()
        };
        Box::new(view)
    }

    fn layout(&self) -> Box<dyn Layout> {
        let child_layout = self.text.layout();
        let mut layout = BlockLayout::new(child_layout);
        layout.id = self.id;
        Box::new(layout)
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![&self.text]
    }

    fn traverse(&self, f: &mut dyn FnMut(&dyn Widget)) {
        f(&self.text);
        self.text.traverse(f);
    }

    fn traverse_mut(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut self.text);
        self.text.traverse_mut(f);
    }
}
