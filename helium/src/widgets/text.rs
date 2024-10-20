use crate::{
	app::events::Event, impl_interative, layout::{IntrinsicSize, Layout, WidgetSize}, surface::{text::TextSurface, Surface} 
};
use super::{Widget, WidgetBody};

#[derive(Debug)]
pub struct Text{
	pub text:String,
	pub font_size:u8,
	pub events: Vec<Event<Self>>
}

impl Text {
	pub fn new(text:&str) -> Self{
		Self { 
			text:text.into(), 
			font_size:16,
			events: Vec::new()
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

		let layout = Layout::Block { padding: 0};
		let intrinsic_size = IntrinsicSize{
			width:WidgetSize::Fixed(size.width),
			height:WidgetSize::Fixed(size.height)
		};

		WidgetBody{
			surface,
			layout,
			intrinsic_size,
			..Default::default()
		}
	}
}
