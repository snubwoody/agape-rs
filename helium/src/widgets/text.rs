use crystal::{BoxSizing, EmptyLayout, Layout};
use helium_core::color::Color;
use helium_macros::hex;
use crate::surface::{text::TextSurface, Surface} ;
use super::{Widget, WidgetBody};

// TODO add background, foreground color,padding and border radius
// TODO probably crate a rich text then make text a tuple struct or a function
pub struct Text{
	text:String,
	font_size:u8,
	color:Color
}

impl Text {
	pub fn new(text:&str) -> Self{
		Self { 
			text:text.into(), 
			font_size:16,
			color:Color::Hex("#000000")
		}	
	}

	pub fn color(mut self,color:Color) -> Self{
		self.color = color;
		self
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
			self.font_size,
			&self.color
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
