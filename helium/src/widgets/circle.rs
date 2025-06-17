use super::{LayoutConfig, Widget, WidgetBody};
use crystal::{BoxSizing, EmptyLayout, Layout};
use helium_core::{Color, GlobalId, IntoColor, Rgba, colors::BLACK};
use helium_renderer::IntoSurface;

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

    fn build(&self, _renderer: &mut helium_renderer::Renderer) -> WidgetBody {
        let primitive = helium_renderer::CircleSurface::new(self.diameter as f32)
            .color(self.color.clone())
            .into_surface();

        // FIXME test this and add the size here
        let layout = LayoutConfig::empty();

        WidgetBody {
            id: self.id.clone(),
            primitive,
            children: vec![],
            layout,
        }
    }

    fn layout(&self, _: &mut helium_renderer::Renderer) -> Box<dyn Layout> {
        let mut layout = EmptyLayout::new();
        layout.intrinsic_size.width = BoxSizing::Fixed(self.diameter as f32);
        layout.intrinsic_size.height = BoxSizing::Fixed(self.diameter as f32);
        layout.id = self.id.clone();

        Box::new(layout)
    }

    fn draw(&self, layout: &dyn Layout, renderer: &mut helium_renderer::Renderer) {
        renderer.draw([helium_renderer::CircleSurface::new(layout.size().width)
            .position(layout.position().x, layout.position().y)
            .color(self.color.clone())]);
    }
}
