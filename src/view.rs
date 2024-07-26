use glium::{
	glutin::surface::WindowSurface, Display, Program, Surface,
};
use winit::window::Window;
use crate::{widgets::Widget, RenderContext};

// TODO change these fields to private and make a new impl
/// A page-like structure that holds multiple widgets below it and renders them.  
/// It can only have one [`Widget`] child
pub struct View<W:Widget>{
	pub context:RenderContext,
	pub child:W
}

impl<W> View<W> where W:Widget {
	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		window:&Window,
	){
		// Create a frame that will be drawn to
		let mut frame = display.draw();
		frame.clear_color(1.0, 1.0, 1.0, 1.0);

		//Render the children, passing the objects down the widget tree
		self.child.render(display,&mut frame,window,&self.context);

		//Swap the buffers
		frame.finish().unwrap();
	}
}
