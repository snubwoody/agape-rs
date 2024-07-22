use glium::{
	glutin::surface::WindowSurface, index, Display, Frame, Program, Surface, VertexBuffer
};
use winit::window::Window;
use crate::widgets::Widget;
use crate::Vertex;


/// A rect is the underlying structure of every thing
/// that can be drawn to the screen.  
/// It itself is also a [`Widget`]
pub struct Rect{
	pub x:i32,
	pub y:i32,
	pub width:i32,
	pub height:i32,
	pub colour:[f32;4]
}

impl Rect {
	pub fn new(x:i32,y:i32,width:i32,height:i32,colour:[f32;4]) -> Self {
		Self{x, y, width, height, colour }
	}

	pub fn to_vertices(&self) -> Vec<Vertex>{

		let vertex1 = Vertex::new(self.x, self.y,self.colour); //Top left
		let vertex2 = Vertex::new(self.x+self.width, self.y,self.colour); // Top right
		let vertex3 = Vertex::new(self.x, self.y+self.height,self.colour); //Bottom left
		let vertex4 = Vertex::new(self.x+self.width, self.y,self.colour); //Top right
		let vertex5 = Vertex::new(self.x, self.y+self.height,self.colour); // Bottom left
		let vertex6 = Vertex::new(self.x+self.width, self.y+self.height,self.colour); //Bottom right

		return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
	}

	pub fn translate(&mut self,x:i32,y:i32){
		self.x += x;
		self.y += y;
	}

	pub fn colour(&self,colour:[f32;4]) -> Self{
		Self{
			x:self.x,
			y:self.y,
			width:self.width,
			height:self.height,
			colour:colour
		}
	}
}

impl Widget for Rect {
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		program:&Program,
	){
		let vertices:Vec<Vertex> = self.to_vertices();
		let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
		let indices = index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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
			&Default::default()).unwrap();
	}

	fn set_position(&mut self,x:i32,y:i32){
		self.x = x;
		self.y = y;
	}

	fn size(&mut self) -> [i32;2] {
		return [self.width,self.height];
	}
}
