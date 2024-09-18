use crate::{
	colour::Colour,
	surface::rect::RectSurface, 
	widgets::Widget,
	layout::Layout
};
use super::WidgetBody;
/* 
/// A container [`Widget`] that wraps its child
#[derive(Debug)]
pub struct Container{
	pub padding:u32,
	pub colour:Colour,
	pub child:Box<dyn Widget>
}

impl Container {
	pub fn new(child:impl Widget + 'static) -> Self{
		Container { 
			padding:0, 
			colour:Colour::Rgb(255, 255, 255), 
			child:Box::new(child)
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

impl Widget for Container {
	fn build(&self) -> WidgetBody {
		let surface = Box::new(
			RectSurface{
				colour:self.colour.clone(),
				..Default::default()
			}
		);
		
		let layout = Layout::Single { padding: 12 };
		let child = self.child.build();

		WidgetBody{
			surface,
			layout,
			children:vec![Box::new(child)],
			..Default::default()
		}
	}

	fn get_children(self) -> Vec<Box<dyn Widget>> {
		return vec![self.child];
	}
}
 */