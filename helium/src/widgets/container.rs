use crate::{impl_style, widgets::Widget};
use crystal::{BlockLayout, Layout};
use helium_core::{Color, GlobalId, Rgba};
use helium_renderer::IntoSurface;
use nanoid::nanoid;

use super::{LayoutConfig, LayoutType, WidgetBody};

/// A container [`Widget`] that wraps its child
pub struct Container<W> {
    id: GlobalId,
    color: Color<Rgba>,
    child: W,
    corner_radius: u32,
    padding: u32,
}

impl<W> Container<W>
where
    W: Widget,
{
    pub fn new(child: W) -> Self {
        Container {
            id: GlobalId::new(),
            color: Color::rgb(255, 255, 255),
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
    fn id(&self) -> GlobalId {
        self.id
    }

    fn build(&self, renderer: &mut helium_renderer::Renderer) -> WidgetBody {
        let primitive = helium_renderer::RectSurface::new(0.0, 0.0)
            .color(self.color.clone())
            .corner_radius(self.corner_radius as f32)
            .into_surface();

        let child = self.child.build(renderer);

        let layout = LayoutConfig::block().padding(self.padding);

        WidgetBody {
            id: self.id.clone(),
            primitive,
            layout,
            children: vec![Box::new(child)],
        }
    }

    fn layout(&self, renderer: &mut helium_renderer::Renderer) -> Box<dyn Layout> {
        let child_layout = self.child.layout(renderer);
        let mut layout = BlockLayout::new(child_layout);
        layout.id = self.id.clone();
        layout.padding = self.padding;
        Box::new(layout)
    }

    fn draw(&self, layout: &dyn Layout, renderer: &mut helium_renderer::Renderer) {
        let primitive =
            helium_renderer::RectSurface::new(layout.size().width, layout.size().height)
                .position(layout.position().x, layout.position().y)
                .color(self.color.clone())
                .corner_radius(self.corner_radius as f32);

        renderer.draw([primitive]);
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![&self.child]
    }
}
