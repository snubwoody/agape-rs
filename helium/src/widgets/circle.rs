use helium_core::color::Color;
use nanoid::nanoid;

use crate::{layout::{BlockLayout, IntrinsicSize}, surface::circle::CircleSurface};

use super::{Widget, WidgetBody};


#[derive(Debug,Clone)]
pub struct Circle{ // TODO add a child maybe
	diameter:u32,
	color:Color
}

impl Circle {
	pub fn new(diameter:u32,color:Color) -> Self{
		Self{ diameter,color }
	}
}

impl Widget for Circle {
	fn build(&self) -> WidgetBody {
		let surface = CircleSurface::new(self.diameter,self.color.clone());
		let mut layout = BlockLayout::new(0);
		layout.intrinsic_size(
			IntrinsicSize::new().fixed(self.diameter, self.diameter)
		);

		WidgetBody { 
			surface:Box::new(surface),
			layout:Box::new(layout),
			..Default::default()
		}
	}
}