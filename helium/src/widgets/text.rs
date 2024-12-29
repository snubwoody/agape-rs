use crystal::{BoxSizing, EmptyLayout, Layout};
use crate::surface::{text::TextSurface, Surface} ;
use super::{Widget, WidgetBody};

// TODO probably crate a rich text then make text a tuple struct or a function
pub struct Text{
	text:String,
	font_size:u8,
}

impl Text {
	pub fn new(text:&str) -> Self{
		Self { 
			text:text.into(), 
			font_size:16,
		}	
	}

	/// Set the font size
	pub fn font_size(mut self,size:u8) -> Self{
		self.font_size = size;
		self
	}
}

impl Widget for Text {
	fn build(&self) -> (WidgetBody,Box<dyn Layout>) {
		// Create the text surface to be rendered
		let textsurface = TextSurface::new(
			self.text.as_str(),
			self.font_size
		);

		let size = textsurface.get_size();
		let surface = Box::new(textsurface);

		let body = WidgetBody{
			surface,
			..Default::default()
		};

		let mut layout = EmptyLayout::new();
		layout.intrinsic_size.width = BoxSizing::Fixed(size.width);
		layout.intrinsic_size.height = BoxSizing::Fixed(size.height);
		layout.id = body.id.clone();

		(body,Box::new(layout))
	}
}
