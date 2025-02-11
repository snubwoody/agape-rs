use super::Widget;
use crystal::{BoxSizing, EmptyLayout, Layout};
use helium_core::Color;

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

    fn layout(&self, _: &mut helium_renderer::Renderer) -> Box<dyn Layout> {
        let mut layout = EmptyLayout::new();
        layout.intrinsic_size.width = BoxSizing::Fixed(self.diameter as f32);
        layout.intrinsic_size.height = BoxSizing::Fixed(self.diameter as f32);
        layout.id = self.id.clone();

        Box::new(layout)
    }

    fn draw(&self, layout: &dyn Layout, renderer: &mut helium_renderer::Renderer) {
        renderer.draw([helium_renderer::Circle::new(layout.size().width)
            .position(layout.position().x, layout.position().y)
            .color(self.color)]);
    }
}
