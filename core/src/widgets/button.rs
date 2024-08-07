use crate::{
	colour::Colour, 
	surface::Surface, 
	widgets::WidgetBody,
	layout::Layout
};
use super::Widget;

#[derive(Debug)]
pub struct Button{
	pub text:String
}

impl Widget for Button {
	fn build(&self) -> WidgetBody {
		let surface = Surface::new(0, 0, 500, 500, Colour::Rgb(255, 25, 255));
		let layout = Layout::Single { padding: 0 };
		WidgetBody { surface,layout,children:vec![] }
	}
}