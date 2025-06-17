use super::Widget;
use crystal::{BoxSizing, EmptyLayout, Layout};
use helium_core::{Color, GlobalId, IntoColor, Rgba, colors::BLACK};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Circle {
    id: GlobalId,
    diameter: u32,
    color: Color<Rgba>,
}

impl Circle {
    pub fn new(diameter: u32) -> Self {
        Self {
            id: GlobalId::new(),
            diameter,
            color: BLACK,
        }
    }

    pub fn color(mut self, color: impl IntoColor<Rgba>) -> Self {
        self.color = color.into_color();
        self
    }
}

impl Widget for Circle {
    fn id(&self) -> GlobalId {
        self.id
    }


    fn layout(&self) -> Box<dyn Layout> {
        let mut layout = EmptyLayout::new();
        layout.intrinsic_size.width = BoxSizing::Fixed(self.diameter as f32);
        layout.intrinsic_size.height = BoxSizing::Fixed(self.diameter as f32);
        layout.id = self.id;

        Box::new(layout)
    }
}
