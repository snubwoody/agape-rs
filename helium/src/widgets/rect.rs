use super::{LayoutConfig, LayoutType, Widget, WidgetBody};
use crate::Color;
use crystal::{BoxSizing, EmptyLayout, IntrinsicSize, Layout};
use helium_core::{colors::WHITE, IntoColor, Rgba};
use helium_renderer::IntoSurface;
use nanoid::nanoid;

// TODO add BoxStyle struct
/// A simple rectangle
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Rect {
    id: String,
    intrinsic_size: crystal::IntrinsicSize,
    color: Color<Rgba>,
    corner_radius: u32,
}

impl Rect {
    pub fn new(width: f32, height: f32) -> Self {
        let intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(width),
            height: BoxSizing::Fixed(height),
        };

        Self {
            id: nanoid!(),
            color: WHITE,
            intrinsic_size,
            corner_radius: 0,
        }
    }

    pub fn color(mut self, color: impl IntoColor<Rgba>) -> Self {
        self.color = color.into_color();
        self
    }

    /// Set the corner radius
    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

	/// Set the intrinsic width to `BoxSixing::Flex`.
    pub fn flex_width(mut self, factor: u8) -> Self {
        self.intrinsic_size.width = BoxSizing::Flex(factor);
        self
    }

	/// Set the intrinsic height to `BoxSixing::Flex`.
    pub fn flex_height(mut self, factor: u8) -> Self {
        self.intrinsic_size.height = BoxSizing::Flex(factor);
        self
    }
}

impl Widget for Rect {
    fn id(&self) -> &str {
        &self.id
    }

	fn build(&self,_renderer: &mut helium_renderer::Renderer) -> WidgetBody {
		let primitive = helium_renderer::RectSurface::new(100.0, 100.0)
			.color(self.color.clone())
			.into_surface();

		let layout = LayoutConfig::new()
			.layout(LayoutType::EmptyLayout)
			.intrinsic_size(self.intrinsic_size);

		WidgetBody { 
			id: self.id.clone(), 
			layout, 
			primitive, 
			children: vec![]
		}
	}

    fn layout(&self, _: &mut helium_renderer::Renderer) -> Box<dyn Layout> {
        let mut layout = EmptyLayout::new();
        layout.intrinsic_size = self.intrinsic_size;
        layout.id = self.id.clone();

        Box::new(layout)
    }

    fn draw(&self, layout: &dyn Layout, renderer: &mut helium_renderer::Renderer) {
        renderer.draw([
            helium_renderer::RectSurface::new(layout.size().width, layout.size().height)
                .position(layout.position().x, layout.position().y)
                .color(self.color.clone()),
        ]);
    }
}
