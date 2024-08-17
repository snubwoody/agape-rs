use crate::{
	colour::Colour,
	surface::Surface, 
	widgets::Widget,
	layout::Layout
};
use super::WidgetBody;

/// A container [`Widget`] that can only have one child
#[derive(Debug,Clone,Copy)]
pub struct Container<W:Widget>{
	pub padding:u32,
	pub colour:Colour,
	pub child:W
}

impl<W> Container<W> where W:Widget {
	pub fn new(child:W) -> Self{
		Container { 
			padding:0, 
			colour:Colour::Rgb(255, 255, 255), 
			child 
		}
	}

	pub fn colour(mut self,colour:Colour) -> Self{
		self.colour = colour;
		self
	}

	pub fn padding(mut self,padding:u32) -> Self{
		self.padding = padding;
		self
	}
}

impl<W> Widget for Container<W> where W:Widget {
	fn build(&self) -> WidgetBody {
		let surface = Surface{colour:self.colour,..Default::default()};
		let layout = Layout::Single { padding: 12 };
		let child = self.child.build();

		WidgetBody{
			surface,
			layout,
			children:vec![Box::new(child)],
			..Default::default()
		}
	}
}






