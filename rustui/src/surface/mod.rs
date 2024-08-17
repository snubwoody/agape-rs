pub mod rect;
pub mod text;
use glium::{
	glutin::surface::WindowSurface, 
	index, 
	Blend, 
	DrawParameters, 
	Surface as GliumSurface, 
	VertexBuffer
};
use crate::{app::view::RenderContext, colour::Colour, utils::Bounds, vertex::Vertex};

pub trait Surface {
	fn draw(
		&mut self,
		display:&glium::Display<WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		program:&RenderContext
	);

	/// Set the position of the [`Widget`]
	fn position(&mut self, x:f32,y:f32);	
	
	/// Get the [`Widget`] position
	fn get_position(&self) -> (f32,f32);

	/// Set the size of the [`Widget`]
	fn size(&mut self,width:u32,height:u32);

	/// Get the size of the [`Widget`]
	fn get_size(&self) -> (u32,u32);

	/// Get the bounds of the [`Widget`]
	fn get_bounds(&self) -> Bounds;
}

