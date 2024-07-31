use crate::{
	colour::rgb, 
	 
	surface::Surface, 
	view::RenderContext, 
	widgets::Widget
};
use super::SizeContraint;

/// A container [`Widget`] that can only have one child
pub struct Container<W:Widget>{
	surface:Surface,
	child:W
}

impl<W:Widget> Container<W>{
	pub fn new(child:W) -> Self{
		let surface = Surface::new(0, 0, 0, 0, rgb(255, 255, 255), SizeContraint::Fit);

		Self {
			surface,
			child
		}
	}
}

impl<W:Widget> Widget for Container<W> {
	fn render(
		&mut self,
		display:&glium::Display<glium::glutin::surface::WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		context:&RenderContext
	) {
		self.surface.render(display, frame, window, &context.surface_program);
		
		self.child.render(display, frame, window,context);
	}

	fn position(&mut self,x:i32,y:i32) {
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


