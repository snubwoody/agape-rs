use super::Widget;
use crate::surface::Primitive;
use crate::Color;
use crystal::{BoxSizing, EmptyLayout, IntrinsicSize, Layout};
use nanoid::nanoid;

// TODO change size to u32
/// A simple rectangle
pub struct Rect {
    id: String,
    width: f32,
    height: f32,
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
            width,
            height,
            color,
            intrinsic_size,
            corner_radius: 0,
        }
    }

    /// Set th corner radius
    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

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

	fn primitive(&self) -> Primitive {
		Primitive::Rect { 
			id: self.id.clone(), 
			corner_radius: self.corner_radius, // TODO add corner radius 
			color: self.color 
		}	
	}
}
