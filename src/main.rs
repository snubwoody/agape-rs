mod widgets;
use widgets::{VStack, View};
use crate::widgets::{
	Rect,
	create_program
};

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
	
	let program = create_program(&display);
	let mut box1 = Rect::new(0, 0, 300, 50, rgb(100, 250, 230));
	let mut box2 = Rect::new(0, 0, 300, 50, rgb(100, 25, 230));
	let mut box3 = Rect::new(0, 0, 300, 50, rgb(100, 25, 23));
	let mut box4 = Rect::new(0, 0, 300, 50, rgb(10, 25, 230));
	let mut box5 = Rect::new(0, 0, 300, 50, rgb(10, 25, 23));

	let mut page = View{
		child:VStack{
			children:vec![&mut box1, &mut box2,&mut box3,&mut box4,&mut box5]
		}
	};

	let _ = event_loop.run(move | event,window_target|{
		match event {
			winit::event::Event::WindowEvent{event,..} => match event{
				winit::event::WindowEvent::CloseRequested => window_target.exit(),
				winit::event::WindowEvent::RedrawRequested => {
	
					page.render(&display, &window, &program);

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

#[derive(Debug,Clone,Copy)]
struct Vertex{
	position: [i32;2],
	colour:[f32;4]
}

impl Vertex {
	fn new(x:i32,y:i32,colour:[f32;4]) -> Self{
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

implement_vertex!(Vertex,position,colour);

fn rgb(r:i32,g:i32,b:i32) -> [f32;4]{
	let red = map(r as f32, [0.0,255.0], [0.0,1.0]);
	let green = map(g as f32, [0.0,255.0], [0.0,1.0]);
	let blue = map(b as f32, [0.0,255.0], [0.0,1.0]);
	return [red,green,blue,1.0]
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

