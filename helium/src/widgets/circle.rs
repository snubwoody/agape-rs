use super::{LayoutConfig, Widget, WidgetBody};
use crystal::{BoxSizing, EmptyLayout, Layout};
use helium_core::{colors::BLACK, Color, IntoColor, Rgba};
use helium_renderer::IntoPrimitive;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Circle {
    id: String,
    diameter: u32,
    color: Color<Rgba>,
}

impl Circle {
    pub fn new(diameter: u32) -> Self {
        Self {
            id: nanoid::nanoid!(),
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
    fn id(&self) -> &str {
        &self.id
    }

	fn build(&self,_renderer: &mut helium_renderer::Renderer) -> WidgetBody {
		let primitive = helium_renderer::Circle::new(self.diameter as f32)
			.color(self.color.clone())
			.into_primitive();

		// FIXME test this and add the size here
		let layout = LayoutConfig::empty();

		WidgetBody{
			id: self.id.clone(),
			primitive,
			children:vec![],
			layout
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
        renderer.draw([helium_renderer::Circle::new(layout.size().width)
            .position(layout.position().x, layout.position().y)
            .color(self.color.clone())]);
    }
}
