use super::Widget;
use crate::{view::RectView, Color};
use crystal::{BoxSizing, EmptyLayout, IntrinsicSize, Layout};
use nanoid::nanoid;

/// A simple rectangle
pub struct Rect {
    id: String,
    intrinsic_size: crystal::IntrinsicSize,
    color: Color,
    corner_radius: u32,
}

impl Rect {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        let intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(width),
            height: BoxSizing::Fixed(height),
        };

        Self {
            id: nanoid!(),
            color,
            intrinsic_size,
            corner_radius: 0,
        }
    }

    /// Set the corner radius
    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

	// TODO replace with impl_widget!()
    pub fn flex_width(mut self, factor: u8) -> Self {
        self.intrinsic_size.width = BoxSizing::Flex(factor);
        self
    }

    pub fn flex_height(mut self, factor: u8) -> Self {
        self.intrinsic_size.height = BoxSizing::Flex(factor);
        self
    }
}

impl Widget for Rect {
    fn layout(&self) -> Box<dyn Layout> {
        let mut layout = EmptyLayout::new();
        layout.intrinsic_size = self.intrinsic_size;
        layout.id = self.id.clone();

        Box::new(layout)
    }

	fn view(&self) -> Box<dyn crate::view::View> {
		Box::new(
			RectView::new(&self.id)
				.color(self.color)
				.corner_radius(self.corner_radius)
		)
	}
}
