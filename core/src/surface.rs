use glium::{
	glutin::surface::WindowSurface, index, Blend, DrawParameters, Surface as GliumSurface, VertexBuffer
};
use crate::{colour::Colour, vertex::Vertex};

/// This is a primitive that draws to the screen. This holds
/// essential information about the [`Widget`], ie.
/// the colour, coordinates and size.
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Surface{
	pub x:i32,
	pub y:i32,
	pub width:i32,
	pub height:i32,
	pub colour:Colour,
}

impl Surface {
	pub fn new(x:i32,y:i32,width:i32,height:i32,colour:Colour) -> Self{
		Self { x,y,width,height,colour }
	}

	pub fn render(
		&mut self,
		display:&glium::Display<WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		program:&glium::Program,
	) {
		let vertices:Vec<Vertex> = self.to_vertices();
		let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
		let indices = index::NoIndices(glium::index::PrimitiveType::TrianglesList);

		let params = DrawParameters{
			blend:Blend::alpha_blending(),
			..Default::default()
		};

		let screen_width = window.inner_size().width as f32;
		let screen_height = window.inner_size().height as f32;

		frame.draw(
			&vertex_buffer, 
			&indices, 
			&program, 
			&uniform! {
				width:screen_width,
				height:screen_height,
			},
			&params
		).unwrap();
	}

	pub fn to_vertices(&self) -> Vec<Vertex>{

		let colour = self.colour.normalize();

		let vertex1 = Vertex::new(self.x, self.y,colour); //Top left
		let vertex2 = Vertex::new(self.x+self.width, self.y,colour); // Top right
		let vertex3 = Vertex::new(self.x, self.y+self.height,colour); //Bottom left
		let vertex4 = Vertex::new(self.x+self.width, self.y,colour); //Top right
		let vertex5 = Vertex::new(self.x, self.y+self.height,colour); // Bottom left
		let vertex6 = Vertex::new(self.x+self.width, self.y+self.height,colour); //Bottom right

		return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
	}
}
