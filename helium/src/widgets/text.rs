use nanoid::nanoid;

use crate::{
	app::events::Event, layout::{self, BlockLayout, IntrinsicSize, Layout, WidgetSize}, surface::{text::TextSurface, Surface} 
};
use super::{Widget, WidgetBody};

pub struct Text{
	id:String,
	text:String,
	font_size:u8,
}

impl Text {
	pub fn new(text:&str) -> Self{
		Self { 
			id:nanoid!(),
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
	fn build(&self) -> WidgetBody {
		// Create the text surface to be rendered
		let textsurface = TextSurface::new(
			self.text.as_str(),
			self.font_size
		);

		let size = textsurface.get_size();
		let surface = Box::new(textsurface);

		let intrinsic_size = IntrinsicSize{
			width:WidgetSize::Fixed(size.width),
			height:WidgetSize::Fixed(size.height)
		};

		let mut layout = BlockLayout::new(0);
		layout.intrinsic_size(intrinsic_size);

		WidgetBody{
			id:self.id.clone(),
			surface,
			layout:Box::new(layout),
			..Default::default()
		}
	}
}
