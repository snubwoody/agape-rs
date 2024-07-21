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

/// A page-like structure that holds multiple widgets below it and renders them
pub struct View<'a>{
	pub child:VStack<'a>
}

impl<'a> View<'a> {
	pub fn render(
		&mut self,
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

/// A widget that arranges children in a vertical list   
pub struct VStack<'a>{
	pub children:Vec<&'a mut Rect>
}


//TODO there might be unnecessary mutability here
impl<'a> VStack<'a> {
	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		program:&Program,
	){
		let mut spacing = 20;
		let mut offset = 0;
		self.children.iter_mut().for_each(|child|{
			let y_position = offset;
			child.set_position(0, y_position);
			child.render(display, frame, window, program);
			offset += 20 + child.height;
		});
	}
}



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

	pub fn set_position(&mut self,x:i32,y:i32){
		self.x = x;
		self.y = y;
	}

	pub fn render(
		&self,
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


pub fn create_program(display:&Display<WindowSurface>) -> Program {
	let vertex_shader = fs::read_to_string("shaders/triangle.vert").unwrap();
	let fragment_shader = fs::read_to_string("shaders/triangle.frag").unwrap();
	let program = glium::Program::from_source(display, vertex_shader.as_str(), fragment_shader.as_str(), None).unwrap();
	return program
}