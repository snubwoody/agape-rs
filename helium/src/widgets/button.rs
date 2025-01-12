use super::Widget;
use crate::view::{RectView, View};
use crystal::{BlockLayout, Layout};
use helium_core::color::Color;

/// A simple button.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Button<W> {
    id: String,
    color: Color,
    padding: u32,
    corner_radius: u32,
    child: W,
}

impl<W: Widget> Button<W> {
    pub fn new(child: W) -> Self {
        Self {
            id: nanoid::nanoid!(),
            color: Color::Hex("#615fff"),
            padding: 12,
            corner_radius: 0,
            child,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }
}

impl<W: Widget> Widget for Button<W> {
    fn layout(&self) -> Box<dyn Layout> {
        let child_layout = self.child.layout();
        let mut layout = BlockLayout::new(child_layout);
        layout.id = self.id.clone();
        layout.padding = self.padding;
        Box::new(layout)
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![&self.child]
    }

    fn view(&self) -> Box<dyn View> {
        Box::new(
            RectView::new(&self.id)
                .color(self.color)
                .corner_radius(self.corner_radius),
        )
    }
}
