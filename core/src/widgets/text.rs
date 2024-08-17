use crate::text::TextSurface;
use super::{Widget, WidgetBody};

#[derive(Debug)]
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

	pub fn font_size(mut self,size:u8) -> Self{
		self.font_size = size;
		self
	}
}

impl Widget for Text {
	fn build(&self) -> super::WidgetBody {
		let surface = TextSurface::new(
			0, 
			0, 
			self.text.as_str(),
			"#000" , 
			self.font_size
		);
		WidgetBody{
			..Default::default()
		}
	}
}


