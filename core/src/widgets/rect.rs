use glium::{
	glutin::surface::WindowSurface, Display, Frame, 
};
use properties::Drawable;
use winit::window::Window;
use crate::colour::Colour;
use crate::{widgets::Widget,Surface};
use crate::view::RenderContext;

/// A simple rectangle
#[derive(Debug,Clone, Copy)]
#[derive(Drawable)]
pub struct Rect{
	surface:Surface
}

impl Rect {
	pub fn new(x:i32,y:i32,width:i32,height:i32,colour:Colour) -> Self {
		Self{
			surface:Surface::new(x,y,width,height,colour)
		}
	}
}

impl Widget for Rect {
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		self.surface.render(display, frame, window, &context.surface_program);
	}		
}
