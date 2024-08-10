use crate::{
	colour::Colour, 
	surface::Surface, 
	widgets::WidgetBody,
	layout::Layout
};
use super::Widget;

pub struct Button{
	pub text:String,
	pub colour:Colour,
	pub on_click:Option<Box<dyn Fn()>>
}

impl Button {
	pub fn new(text:&str) -> Self {
		Self { 
			text:text.into(), 
			colour:Colour::Rgb(255, 255, 255),
			on_click:None 
		}
	}

	pub fn colour(mut self,colour:Colour) -> Self {
		self.colour = colour;
		self
	}

	pub fn on_click(mut self,f:impl Fn() + 'static) -> Self{
		self.on_click = Some(Box::new(f));
		self
	}
}

impl Widget for Button {
	fn build(self) -> WidgetBody {
		let surface = Surface::new(0, 0, 200, 70, Colour::Rgb(255, 225, 255));
		let layout = Layout::SingleChild { width: 250, height: 70 };
		let func;

		match self.on_click {
			Some(function) => func = function,
			None => func = Box::new(||{})
		}

		WidgetBody { 
			surface,
			layout,
			children:vec![],
			events:vec![func] 
		}

	}
}