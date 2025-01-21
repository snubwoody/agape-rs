use super::Widget;
use crate::view::CircleView;
use crystal::{BoxSizing, EmptyLayout, Layout};
use helium_core::color::Color;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Circle {
    id: String,
    diameter: u32,
    color: Color,
}

impl Circle {
    pub fn new(diameter: u32) -> Self {
        Self {
            id: nanoid::nanoid!(),
            diameter,
            color: Color::default(),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Widget for Circle {
    fn id(&self) -> &str {
        &self.id
    }

    fn tick(&mut self, elements: &[crate::events::Element]) {}

    fn layout(&self) -> Box<dyn Layout> {
        let mut layout = EmptyLayout::new();
        layout.intrinsic_size.width = BoxSizing::Fixed(self.diameter as f32);
        layout.intrinsic_size.height = BoxSizing::Fixed(self.diameter as f32);
        layout.id = self.id.clone();

        Box::new(layout)
    }

    fn view(&self) -> Box<dyn crate::view::View> {
        Box::new(CircleView::new(&self.id).color(self.color))
    }
}
