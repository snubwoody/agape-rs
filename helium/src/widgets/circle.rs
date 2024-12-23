use crystal::{BoxSizing, EmptyLayout};
use helium_core::color::Color;
use crate::surface::circle::CircleSurface;
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
		let mut layout = EmptyLayout::new();
		layout.intrinsic_size.width = BoxSizing::Fixed(self.diameter as f32);
		layout.intrinsic_size.height = BoxSizing::Fixed(self.diameter as f32);
		

		WidgetBody { 
			surface:Box::new(surface),
			layout:Box::new(layout),
			..Default::default()
		}
	}
}