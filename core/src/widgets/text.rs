use crate::{layout::Layout, surface::Surface, text::TextSurface};
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

	pub fn font_size(mut self,size:u8) -> Self{
		self.font_size = size;
		self
	}
}

impl Widget for Text {
	fn build(&self) -> WidgetBody {
		let textsurface = TextSurface::new(
			self.text.as_str(),
			"#000000" , 
			self.font_size
		);
		let size = textsurface.get_size();
		let surface = Box::new(textsurface);

		let layout = Layout::SingleChild { width: size.0, height: size.1 };

		WidgetBody{
			surface,
			layout,
			..Default::default()
		}
	}
}


