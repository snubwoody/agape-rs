use std::fs;
use glium::{
	glutin::surface::WindowSurface, Display, Program,
};
use winit::{
	event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::Window
};
use crate::widgets::Widget;
use crate::view::{View,RenderContext};


///TODO
pub struct App<W:Widget>{
	event_loop:EventLoop<()>,
	window:Window,
	display:Display<WindowSurface>,
	views:Vec<View<W>>,
	context:RenderContext,
	index:u32
}

impl<W> App<W> where W:Widget {
	pub fn new() -> Self {
		let event_loop = EventLoop::new().unwrap();

		// Set the control flow to redraw every frame whether or not there
		// are events to process
		event_loop.set_control_flow(ControlFlow::Poll);
		
		let (window,display) = glium::backend::glutin::
		SimpleWindowBuilder::new()
		.build(&event_loop);

		let surface_program = create_program(&display,"shaders/surface.vert","shaders/surface.frag");
		let text_program = create_program(&display,"shaders/text.vert","shaders/text.frag");

		let context = RenderContext::new(surface_program, text_program);

		Self { event_loop,window,display,context,views:vec![],index:0}
	}

	pub fn add_view(mut self,view:View<W>) -> Self{
		self.views.push(view);
		self
	}

	pub fn run(mut self){
		self.event_loop.run(move | event,window_target|{
			match event {
				Event::WindowEvent{event,..} => match event{
					WindowEvent::CloseRequested => window_target.exit(),
					WindowEvent::RedrawRequested => {
						self.views[0].render(&self.display, &self.window,&self.context)
					},
					_ => {}
				}, 
				_ => {}
			}
	
		}).expect("Event loop error occured");
	}
}

pub fn create_program(display:&Display<WindowSurface>,vertex_shader_src:&str,fragment_shader_src:&str) -> Program {
	let vertex_shader = fs::read_to_string(vertex_shader_src).unwrap();
	let fragment_shader = fs::read_to_string(fragment_shader_src).unwrap();
	let program = glium::Program::from_source(display, vertex_shader.as_str(), fragment_shader.as_str(), None).unwrap();
	return program
}
