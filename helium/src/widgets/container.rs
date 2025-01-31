use crate::{impl_style, widgets::Widget};
use crystal::{BlockLayout, Layout};
use helium_core::color::Color;
use nanoid::nanoid;

/// A container [`Widget`] that wraps its child
pub struct Container<W> {
    id: String,
    color: Color,
    child: W, // TODO make this a generic
    corner_radius: u32,
    padding: u32,
}

impl<W> Container<W>
where
    W: Widget,
{
    pub fn new(child: W) -> Self {
        Container {
            id: nanoid!(),
            color: Color::Rgb(255, 255, 255),
            child,
            corner_radius: 0,
            padding: 0,
        }
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    impl_style!();
}

impl<W> Widget for Container<W>
where
    W: Widget,
{
    fn id(&self) -> &str {
        &self.id
    }

    fn layout(&self, renderer: &mut helium_renderer::Renderer) -> Box<dyn Layout> {
        let child_layout = self.child.layout(renderer);
        let mut layout = BlockLayout::new(child_layout);
        layout.id = self.id.clone();
        layout.padding = self.padding;
        Box::new(layout)
    }

    fn draw(&self, layout: &dyn Layout, renderer: &mut helium_renderer::Renderer) {
        renderer.draw([
            helium_renderer::Rect::new(layout.size().width, layout.size().height)
                .position(layout.position().x, layout.position().y)
                .color(self.color)
                .corner_radius(self.corner_radius as f32),
        ]);
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![&self.child]
    }
}
