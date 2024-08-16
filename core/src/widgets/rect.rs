use crate::{colour::Colour};
use crate::Surface;
use super::{Widget, WidgetBody};
use crate::layout::Layout;

/// A simple rectangle
#[derive(Debug,Clone,Copy)]
pub struct Rect{
	pub width:u32,
	pub height:u32,
	pub colour:Colour
}

impl Widget for Rect {
	fn build(&self) -> WidgetBody {
		let layout = Layout::SingleChild{width:self.width,height:self.height};
		
		WidgetBody{ 
			surface:Surface{ 
				width:self.width,
				height:self.height,
				colour:self.colour,
				..Default::default()
			},
			layout,
			children:vec![],
			..Default::default()
		}
	}
}
