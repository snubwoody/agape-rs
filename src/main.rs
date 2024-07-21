use std::fs;
use glium::{
	glutin::surface::WindowSurface,
	Display,
	Program,
	Surface
};
use winit::window::{self, Window};

#[macro_use]
extern crate glium;


fn main() {
	let event_loop = winit::
		event_loop::EventLoopBuilder::new()
		.build()
		.expect("Event loop building");

	let (window,display) = glium::backend::glutin::
		SimpleWindowBuilder::new()
		.build(&event_loop);

	let vertices = rect();
	let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
	
	let program = create_program(&display);

	let _ = event_loop.run(move | event,window_target|{
		match event {
			winit::event::Event::WindowEvent{event,..} => match event{
				winit::event::WindowEvent::CloseRequested => window_target.exit(),
				winit::event::WindowEvent::RedrawRequested => {
					let mut target = display.draw();
					let screen_width = window.inner_size().width as f32;
					let screen_height = window.inner_size().height as f32;
					target.clear_color(1.0, 1.0, 1.0, 1.0);
					target.draw(
						&vertex_buffer, 
						&indices, 
						&program, 
						&uniform! {
							width:screen_width,
							height:screen_height,
						},
						&Default::default()).unwrap();

					target.finish().unwrap();
				}
				_ => {}
			}, 
			winit::event::Event::AboutToWait => {
				window.request_redraw();
			}
			_ => {}
		}

	});
}

fn rect() -> Vec<Vertex>{
	let colour = rgb(255, 25, 107);
	let vertex1 = Vertex::new(200.0, 0.0,colour); //Top left
	let vertex2 = Vertex::new(500.0, 0.0,colour); // Top right
	let vertex3 = Vertex::new(200.0, 500.0,colour); //Bottom left
	let vertex4 = Vertex::new(500.0, 0.0,colour); //Top right
	let vertex5 = Vertex::new(200.0, 500.0,colour); // Bottom left
	let vertex6 = Vertex::new(500.0, 500.0,colour); //Bottom right

	return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
}

fn sq() -> Vec<Vertex>{
	let colour = rgb(255, 25, 107);
	let vertex1 = Vertex::new(-0.5, 0.5,colour); //Top left
	let vertex2 = Vertex::new(0.5, 0.0,colour); // Top right
	let vertex3 = Vertex::new(-0.5, -0.5,colour); //Bottom left
	let vertex4 = Vertex::new(0.5, 0.5,colour); //Top right
	let vertex5 = Vertex::new(-0.5, -0.5,colour); // Bottom left
	let vertex6 = Vertex::new(0.5, -0.5,colour); //Bottom right

	return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
}

struct Rect{
	x:i32,
	y:i32,
	width:i32,
	height:i32,
	colour:[f32;4]
}

impl Rect {
	fn new(x:i32,y:i32,width:i32,height:i32,colour:[f32;4]) -> Self {
		Self{x, y, width, height, colour }
	}

	fn to_vertices(&self,window:&Window) -> Vec<Vertex>{
		let colour = self.colour;
		//let x = map(self.width, [0,window.inner_size().width], -1,1);

		let vertex1 = Vertex::new(-0.5, 0.5,colour); //Top left
		let vertex2 = Vertex::new(0.5, 0.5,colour); // Top right
		let vertex3 = Vertex::new(-0.5, -0.5,colour); //Bottom left
		let vertex4 = Vertex::new(0.5, 0.5,colour); //Top right
		let vertex5 = Vertex::new(-0.5, -0.5,colour); // Bottom left
		let vertex6 = Vertex::new(0.5, -0.5,colour); //Bottom right

		return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
	}
}


fn create_program(display:&Display<WindowSurface>) -> Program {
	let vertex_shader = fs::read_to_string("shaders/triangle.vert").unwrap();
	let fragment_shader = fs::read_to_string("shaders/triangle.frag").unwrap();
	let program = glium::Program::from_source(display, vertex_shader.as_str(), fragment_shader.as_str(), None).unwrap();
	return program
}


#[derive(Debug,Clone,Copy)]
struct Vertex{
	position: [f32;2],
	colour:[f32;4]
}

impl Vertex {
	fn new(x:f32,y:f32,colour:[f32;4]) -> Self{
		let r = colour[0];
		let g = colour[1];
		let b = colour[2];
		let a = colour[3];

		Self { 
			position: [x,y],
			colour:[r,g,b,a]
		}
	}
}

fn rgb(r:i32,g:i32,b:i32) -> [f32;4]{
	let red = map(r as f32, [0.0,255.0], [0.0,1.0]);
	let green = map(g as f32, [0.0,255.0], [0.0,1.0]);
	let blue = map(b as f32, [0.0,255.0], [0.0,1.0]);
	return [red,green,blue,1.0]
}

implement_vertex!(Vertex,position,colour);

/// Map value from one range to another. Any overflow is clipped to the min or max
fn map(mut value:f32,input_range:[f32;2],output_range:[f32;2]) -> f32{
	if value > input_range[1]{
		value = input_range[1]
	}
	else if value < input_range[0] {
		value = input_range[0]
	}

	let scale = (output_range[1] - output_range[0]) / (input_range[1] - input_range[0]);
	let offset = input_range[0]*(scale)+output_range[0];

	return  value * scale + offset;
}

