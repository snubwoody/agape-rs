pub mod rect;
pub mod stack;
pub mod container;
pub mod text;
use std::fmt::Debug;

use glium::{
	glutin::surface::WindowSurface, Display, Frame, 
};
use winit::window::Window;
use crate::app::view::RenderContext;

/// Widget trait that all widgets must inherit from
pub trait Widget:Debug + Drawable{
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	);

	fn get_children(&self) -> Widget;
}

/// Represents anything that's drawable to the screen ie.
/// it must have a size and a position
pub trait Drawable{
	/// Set the position of the [`Widget`]  
	/// Note that positions start from the upper left 
	/// corner
	fn position(&mut self, x:i32,y:i32); 
	
	/// Get the [`Widget`] position
	fn get_position(&self) -> (i32,i32); 

	/// Set the size of the widget
	fn size(&mut self,width:u32,height:u32); 

	/// Get the size of the widget
	fn get_size(&self) -> (u32,u32);
}
