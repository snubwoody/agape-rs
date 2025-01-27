use super::Widget;
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
    fn id(&self) -> &str {
        &self.id
    }

    fn tick(&mut self, elements: &[crate::events::Element]) {}

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

	fn draw(&self,layout:&dyn Layout, renderer:&mut helium_renderer::Renderer) {
		renderer.draw([
			helium_renderer::Rect::new(layout.size().width,layout.size().height)
				.position(layout.position().x, layout.position().y)
				.color(self.color)
				.corner_radius(self.corner_radius as f32)
		]);
	}
}
