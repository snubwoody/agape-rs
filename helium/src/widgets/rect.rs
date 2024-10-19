use crate::surface::rect::RectSurface;
use crate::utils::Size;
use crate::{colour::Colour};
use super::{Widget, WidgetBody};
use crate::layout::{IntrinsicSize, Layout, WidgetSize};

/// A simple rectangle
#[derive(Debug,Clone,PartialEq)]
pub struct Rect{
	pub width:f32,
	pub height:f32,
	pub colour:Colour
}

impl Rect {
	pub fn new(width:f32,height:f32,colour:Colour) -> Self{
		Self { width, height, colour }
	}
}

impl Widget for Rect {
	fn build(&self) -> WidgetBody {
		let layout = Layout::Block { padding: 0 };
		let surface = Box::new(
			RectSurface{ 
				size:Size::new(self.width as f32, self.height as f32),
				colour:self.colour.clone(),
				..Default::default()
			}
		);
		
		WidgetBody{ 
			surface,
			layout,
			children:vec![],
			intrinsic_size:IntrinsicSize{ 
				width: WidgetSize::Fixed(self.width), 
				height: WidgetSize::Fixed(self.height) 
			},
			..Default::default()
		}
	}

	fn get_children(self:Box<Self>) -> Vec<Box<dyn Widget>> {
		vec![]
	}
}
 