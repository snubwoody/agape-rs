pub mod rect;
pub mod text;
pub mod image;
use glium::glutin::surface::WindowSurface;
use crate::{app::view::RenderContext,utils::Bounds};

/// A primitive object that is drawn to the screen
pub trait Surface {
	/// Draw the surface onto the screen
	fn draw(
		&mut self,
		display:&glium::Display<WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		program:&RenderContext
	);

	/// Set the position of the [`Surface`]
	fn position(&mut self, x:f32,y:f32);	
	
	/// Get the [`Surface`] position
	fn get_position(&self) -> (f32,f32);

	/// Set the size of the [`Surface`]
	fn size(&mut self,width:u32,height:u32);

	/// Get the size of the [`Surface`]
	fn get_size(&self) -> (u32,u32);

	/// Get the bounds of the [`Surface`]
	fn get_bounds(&self) -> Bounds;
}

