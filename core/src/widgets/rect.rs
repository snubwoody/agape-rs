use glium::{
	glutin::surface::WindowSurface, Display, Frame, 
};
use properties::Drawable;
use winit::window::Window;
use crate::colour::Colour;
use crate::{widgets::Widget,Surface};
use crate::app::view::RenderContext;
use super::{WidgetBody, WidgetBuilder};

/// A simple rectangle
#[derive(Debug,Clone,Copy)]
pub struct Rect{
	pub width:u32,
	pub height:u32,
	pub colour:Colour
}

impl WidgetBuilder for Rect {
	fn build(&self) -> WidgetBody {
		WidgetBody{ 
			surface:Surface{ 
				x:0, 
				y:0, 
				width:self.width as i32,
				height:self.height as i32,
				colour:self.colour
			} 
		}
	}
}
