use super::{Widget, WidgetBody};
use crate::Color;
use crate::layout::IntrinsicSize;
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
			intrinsic_size:IntrinsicSize::fixed(width, height),
			radius:0
        }
    }

	pub fn fill(mut self) -> Self{
		self.intrinsic_size.fill();
		self
	}

	pub fn fill_width(mut self) -> Self{
		self.intrinsic_size.fill_width();
		self
	}

	pub fn fill_height(mut self) -> Self{
		self.intrinsic_size.fill_height();
		self
	}

	/// Set th border radius
	pub fn corner_radius(mut self,radius:u32) -> Self{
		self.radius = radius;
		self
	}

	/// A shorthand for border radius
	pub fn rounded(){

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

        WidgetBody {
			id:self.id.clone(),
            surface,
            children: vec![],
            intrinsic_size:self.intrinsic_size,
            ..Default::default()
        }
    }

}

