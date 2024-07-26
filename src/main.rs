mod widgets;
mod view;
mod colour;
pub mod surface;
pub mod text;
pub mod vertex;
use widgets::container::Container;
use widgets::stack::{Stack,StackDirection};
use widgets::rect::Rect;
use widgets::text::Text;
use std::fs;
use glium::{
	glutin::surface::WindowSurface, Display, Program,
};
use crate::surface::Surface;
use crate::widgets::Widget;
use crate::view::View;
use crate::colour::rgb;
#[macro_use]
extern crate glium;


fn main() {
	run_app();
}

fn run_app<'a>() {
	let event_loop = winit::
		event_loop::EventLoopBuilder::new()
		.build()
		.expect("Event loop building");

	let (window,display) = glium::backend::glutin::
		SimpleWindowBuilder::new()
		.build(&event_loop);

	let surface_program = create_program(&display,"shaders/triangle.vert","shaders/triangle.frag");
	let text_program = create_program(&display,"shaders/text.vert","shaders/text.frag");

	let mut box1 = Rect::new(0, 0, 100, 50, rgb(100, 250, 230));
	let mut box2 = Rect::new(0, 0, 100, 50, rgb(100, 25, 230));
	let mut box5 = Rect::new(0, 0, 100, 50, rgb(255, 255, 255));
	
	let container = Container::new(300, 100, 20, rgb(20, 250,50), box5);

	let context = RenderContext::new(surface_program, text_program);
	let text = Text::new(0, 0, "Hello", "#000", 18);
	
	let test = vstack!{
		spacing:150,
		width:200,
		height:400,
		box1,
		container,
		text
	};	

	let mut page = View{
		context:context,
		child:test
	};

	let _ = event_loop.run(move | event,window_target|{
		match event {
			winit::event::Event::WindowEvent{event,..} => match event{
				winit::event::WindowEvent::CloseRequested => window_target.exit(),
				winit::event::WindowEvent::RedrawRequested => {

					page.render(&display, &window);
					//render_text(&display,&text_program,&window);

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

/// Contains all the data required for a surface to 
/// be rendered to the screen
struct RenderContext{
	pub surface_program:Program,
	pub text_program:Program,
}

impl RenderContext {
	pub fn new(
		surface_program:Program,
		text_program:Program
	) -> Self {
		Self { surface_program, text_program }
	}
}


pub struct Position{
	x:i32,
	y:i32
}

pub struct Size{
	width:u32,
	height:u32
}


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



pub fn create_program(display:&Display<WindowSurface>,vertex_shader_src:&str,fragment_shader_src:&str) -> Program {
	let vertex_shader = fs::read_to_string(vertex_shader_src).unwrap();
	let fragment_shader = fs::read_to_string(fragment_shader_src).unwrap();
	let program = glium::Program::from_source(display, vertex_shader.as_str(), fragment_shader.as_str(), None).unwrap();
	return program
}
