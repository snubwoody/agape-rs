use super::{Widget, WidgetBody};
use crate::Color;
use crate::surface::rect::RectSurface;
use crate::Size;
use crystal::{BlockLayout, BoxSizing, EmptyLayout, IntrinsicSize, Layout};
use nanoid::nanoid;

// TODO change size to u32
/// A simple rectangle
pub struct Rect{
	id:String,
    width: f32,
    height: f32,
	intrinsic_size:crystal::IntrinsicSize,
    color: Color,
	radius:u32
}

impl Rect {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        let intrinsic_size = IntrinsicSize{
			width:BoxSizing::Fixed(width),
			height:BoxSizing::Fixed(height)
		};

		Self {
			id:nanoid!(),
            width,
            height,
            color,
			intrinsic_size,
			radius:0
        }
    }

	/// Set th border radius
	pub fn corner_radius(mut self,radius:u32) -> Self{
		self.radius = radius;
		self
	}

	pub fn flex_width(mut self,factor:u8) -> Self{
		self.intrinsic_size.width = BoxSizing::Flex(factor);
		self
	}

	pub fn flex_height(mut self,factor:u8) -> Self{
		self.intrinsic_size.height = BoxSizing::Flex(factor);
		self
	}
}

impl Widget for Rect {
    fn build(&self) -> (WidgetBody,Box<dyn Layout>) {
        let surface = Box::new(RectSurface {
            size: Size::new(self.width as f32, self.height as f32),
            color: self.color.clone(),
			corner_radius:self.radius,
            ..Default::default()
        });

		let body = WidgetBody {
			id:self.id.clone(),
            surface,
            children: vec![],
            ..Default::default()
        };

		let mut layout = EmptyLayout::new();
		layout.intrinsic_size = self.intrinsic_size;
		layout.id = body.id.clone();

		(body,Box::new(layout))
    }

}

