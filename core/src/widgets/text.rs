use crate::{layout::Layout, text::TextSurface};
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
		// FIXME set the actual width here
		let surface = Box::new(
			TextSurface::new(
				self.text.as_str(),
				"#000000" , 
				self.font_size
			)
		);

		let layout = Layout::SingleChild { width: 97, height: 12 };

		WidgetBody{
			surface,
			layout,
			..Default::default()
		}
	}
}


