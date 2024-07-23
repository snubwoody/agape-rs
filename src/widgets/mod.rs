pub mod rect;
pub mod stack;
pub mod surface;
pub mod container;
use glium::{
	glutin::surface::WindowSurface, Display, Frame, Program,
};
use winit::window::Window;

/// Widget trait that all widgets must inherit from
pub trait Widget {
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		program:&Program,
	);
	/// Set the position of the [`Widget`]  
	/// Note that positions start from the upper left 
	/// corner
	// TODO change to position
	fn set_position(&mut self,x:i32,y:i32);
	//TODO change to get_size then add function size that sets the size 
	// to be more idiomatic
	///Returns the size
	fn size(&mut self) -> [i32;2];		
}