use super::{Widget, WidgetBody};
use crate::Color;
use crate::layout::{BlockLayout, IntrinsicSize};
use crate::surface::rect::RectSurface;
use crate::Size;
use nanoid::nanoid;

// TODO change size to u32
/// A simple rectangle
pub struct Rect{
	id:String,
    width: u32,
    height: u32,
    color: Color,
	intrinsic_size:IntrinsicSize,
	radius:u32
}

impl Rect {
    pub fn new(width: u32, height: u32, color: Color) -> Self {
        Self {
			id:nanoid!(),
            width,
            height,
            color,
			intrinsic_size:IntrinsicSize::new().fixed(width, height),
			radius:0
        }
    }

	pub fn fill(mut self) -> Self{
		self.intrinsic_size = self.intrinsic_size.fill();
		self
	}

	pub fn fill_width(mut self) -> Self{
		self.intrinsic_size = self.intrinsic_size.fill_width();
		self
	}

	pub fn fill_height(mut self) -> Self{
		self.intrinsic_size = self.intrinsic_size.fill_height();
		self
	}

	/// Set th border radius
	pub fn corner_radius(mut self,radius:u32) -> Self{
		self.radius = radius;
		self
	}
}

impl Widget for Rect {
    fn build(&self) -> WidgetBody {
        let surface = Box::new(RectSurface {
            size: Size::new(self.width as f32, self.height as f32),
            color: self.color.clone(),
			corner_radius:self.radius,
            ..Default::default()
        });

		let mut layout = BlockLayout::new(0);
		layout.intrinsic_size(self.intrinsic_size);

        WidgetBody {
			id:self.id.clone(),
            surface,
            children: vec![],
            ..Default::default()
        }
    }

}

