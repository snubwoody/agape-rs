use glium::{
	glutin::surface::WindowSurface, Display, Program, Surface,
};
use winit::window::Window;
use crate::widgets::Widget;

/// A page-like structure that holds multiple widgets below it and renders them.  
/// It can only have one [`Widget`] child
pub struct View<T>{
	pub child:T
}

impl<T> View<T> where T:Widget {
	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		window:&Window,
		program:&Program,
	){
		let mut frame = display.draw();
		frame.clear_color(1.0, 1.0, 1.0, 1.0);
		self.child.render(display,&mut frame,window,program);
		frame.finish().unwrap();
	}
}