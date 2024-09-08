pub mod rect;
pub mod text;
pub mod image;
use std::fmt::Debug;

use glium::glutin::surface::WindowSurface;
use crate::{app::view::RenderContext,utils::{Bounds,Size}};

/// A primitive object that is drawn to the screen
pub trait Surface:Debug {
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
	
	/// Get the [`Surface`] position.
	fn get_position(&self) -> (f32,f32);

	/// Set the [`Size`] of the [`Surface`].
	fn size(&mut self,width:f32,height:f32);

	/// Get the [`Size`] of the [`Surface`].
	fn get_size(&self) -> Size;

	/// Get the bounds of the [`Surface`]
	fn get_bounds(&self) -> Bounds;
}

