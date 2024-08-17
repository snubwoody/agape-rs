use crate::text::TextSurface;
use super::{Widget, WidgetBody};

#[derive(Debug)]
pub struct Text{
	text:String,
	font_size:u8
}

impl Text {
	pub fn new(text:&str) -> Self{
		Self { 
			text:text.into(), 
			font_size:16 
		}	
	}
}

impl Widget for Text {
	fn build(&self) -> super::WidgetBody {
		let surface = TextSurface::new(
			0, 
			0, 
			self.text.as_str(),
			"#000" , 
			16
		);
		WidgetBody{
			..Default::default()
		}
	}
}


