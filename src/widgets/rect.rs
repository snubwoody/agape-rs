use glium::{
	glutin::surface::WindowSurface, Display, Frame, 
};
use winit::window::Window;
use crate::{widgets::Widget,Surface};
use crate::view::RenderContext;
use super::SizeContraint;

/// A simple rectangle
pub struct Rect{
	surface:Surface
}

impl Rect {
	pub fn new(x:i32,y:i32,width:i32,height:i32,colour:[f32;4]) -> Self {
		
		Self{
			surface:Surface::new(x,y,width,height,colour,SizeContraint::Fixed(width as u32, height as u32))
		}
	}


}

//FIXME replace this with surface
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

	fn position(&mut self,x:i32,y:i32){
		self.surface.x = x;
		self.surface.y = y;
	}


	fn size(&mut self,width:u32,height:u32) {
		self.surface.width = width as i32;
		self.surface.height = height as i32;
	}

	fn get_size(&self) -> (u32,u32) {
		(self.surface.width as u32,self.surface.height as u32)
	}
		
}
