pub mod rect;
pub mod stack;
pub mod container;
pub mod text;
use glium::{
	glutin::surface::WindowSurface, Display, Frame, 
};
use winit::window::Window;

use crate::view::RenderContext;

/// Widget trait that all widgets must inherit from
pub trait Widget {
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	);
	
	/// Set the position of the [`Widget`]  
	/// Note that positions start from the upper left 
	/// corner
	fn position(&mut self,x:i32,y:i32);
	
	//fn get_surface(&self) -> Surface;	

	///Returns the size
	fn get_size(&self) -> [i32;2];
}
