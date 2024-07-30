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
	
	/// Set the size of the widget
	fn size(&mut self,width:u32,height:u32);

	/// Get the size of the widget
	fn get_size(&self) -> (u32,u32);

	fn arrange_widgets(&mut self,max_size:[u32;2]);

}

/// Represents the sizing constraints a widget should have ie.
/// `Fit` the children, `Fill` the parent, `Relative` to the
/// parent as a percentage or a `Fixed` size in pixels
#[derive(Debug,Clone,Copy,PartialEq)] 
pub enum SizeContraint{
	Fill,
	Fit,
	Relative(f32,f32),
	Fixed(u32,u32)
}
