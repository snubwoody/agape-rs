use crate::surface::rect::RectSurface;
use crate::{colour::Colour};
use super::{Widget, WidgetBody};
use crate::layout::Layout;

/// A simple rectangle
#[derive(Debug,Clone,PartialEq, Eq)]
pub struct Rect{
	pub width:u32,
	pub height:u32,
	pub colour:Colour
}

impl Rect {
	pub fn new(width:u32,height:u32,colour:Colour) -> Self{
		Self { width, height, colour }
	}
}

impl Widget for Rect {
	fn build(&self) -> WidgetBody {
		let layout = Layout::SingleChild{width:self.width,height:self.height};
		let surface = Box::new(
			RectSurface{ 
				width:self.width,
				height:self.height,
				colour:self.colour.clone(),
				..Default::default()
			}
		);
		
		WidgetBody{ 
			surface,
			layout,
			children:vec![],
			..Default::default()
		}
	}
}
