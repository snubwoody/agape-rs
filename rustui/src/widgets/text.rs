use crate::{
	layout::Layout, 
	surface::{text::TextSurface, Surface}, 
};
use super::{Widget, WidgetBody};

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Text{
	pub text:String,
	pub font_size:u8
}

impl Text {
	pub fn new(text:&str) -> Self{
		Self { 
			text:text.into(), 
			font_size:16 
		}	
	}

	/// Set the font size
	pub fn font_size(mut self,size:u8) -> Self{
		self.font_size = size;
		self
	}
}

impl Widget for Text {
	fn build(&self) -> WidgetBody {
		// Create the text surface to be rendered
		let textsurface = TextSurface::new(
			self.text.as_str(),
			"#000000" , 
			self.font_size
		);

		let size = textsurface.get_size();
		let surface = Box::new(textsurface);

		let layout = Layout::SingleChild { width: size.width as u32, height: size.height as u32 };

		WidgetBody{
			surface,
			layout,
			..Default::default()
		}
	}
	
	fn get_children(self) -> Vec<Box<dyn Widget>> {
		vec![]
	}
}


