use crate::{
	colour::Colour,
	surface::Surface, 
	widgets::Widget,
	layout::Layout
};
use super::WidgetBody;

/// A container [`Widget`] that can only have one child
#[derive(Debug,Clone,Copy)]
pub struct Container<W:Widget>{
	padding:u32,
	colour:Colour,
	child:W
}

impl<W> Widget for Container<W> where W:Widget {
	fn build(&self) -> WidgetBody {
		let surface = Surface{colour:self.colour,..Default::default()};
		let layout = Layout::Horizontal { spacing: 12, padding: 12 };
		WidgetBody{
			surface,
			layout
		}
	}
}






