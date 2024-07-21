use std::fs;
use glium::{
	glutin::surface::WindowSurface, 
	index, 
	Display, 
	Frame, 
	Program, 
	Surface, 
	VertexBuffer
};
use winit::window::Window;
use crate::Vertex;
pub struct View{
	pub child:Rect
}

impl View {
	pub fn render(
		&self,
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

pub trait Widget {
	fn render(&self);
}

struct VStack<'a>{
	children:Vec<&'a Rect>
}

impl<'a> VStack<'a> {
	pub fn render(){

	}
}



pub struct Rect{
	x:i32,
	y:i32,
	width:i32,
	height:i32,
	colour:[f32;4]
}

impl Rect {
	pub fn new(x:i32,y:i32,width:i32,height:i32,colour:[f32;4]) -> Self {
		Self{x, y, width, height, colour }
	}

	pub fn to_vertices(&self,window:&Window) -> Vec<Vertex>{

		let vertex1 = Vertex::new(self.x, self.y,self.colour); //Top left
		let vertex2 = Vertex::new(self.x+self.width, self.y,self.colour); // Top right
		let vertex3 = Vertex::new(self.x, self.y+self.height,self.colour); //Bottom left
		let vertex4 = Vertex::new(self.x+self.width, self.y,self.colour); //Top right
		let vertex5 = Vertex::new(self.x, self.y+self.height,self.colour); // Bottom left
		let vertex6 = Vertex::new(self.x+self.width, self.y+self.height,self.colour); //Bottom right

		return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
	}

	pub fn render(
		&self,
		display:&Display<WindowSurface>,
		mut frame:&mut Frame,
		window:&Window,
		program:&Program,
	){
		let vertices:Vec<Vertex> = self.to_vertices(window);
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
}


pub fn create_program(display:&Display<WindowSurface>) -> Program {
	let vertex_shader = fs::read_to_string("shaders/triangle.vert").unwrap();
	let fragment_shader = fs::read_to_string("shaders/triangle.frag").unwrap();
	let program = glium::Program::from_source(display, vertex_shader.as_str(), fragment_shader.as_str(), None).unwrap();
	return program
}