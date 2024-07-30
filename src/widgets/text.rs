use crate::{view::RenderContext, Widget};
use crate::text::TextSurface;

#[derive(Debug)]
pub struct Text{
	surface:TextSurface,
	text:String,
	font_size:u8
}

impl Text {
	pub fn new(x:i32,y:i32,text:&str,colour:&str,font_size:u8) -> Self{
		let surface = TextSurface::new(x, y, text, colour, font_size);
		Self { surface, text:text.into(), font_size }	
	}
}

impl Widget for Text {
	fn render(
		&mut self,
		display:&glium::Display<glium::glutin::surface::WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		context:&RenderContext,
	) {
		self.surface.build(display);
		self.surface.render(display, frame, window, &context.text_program);
	}

	//FIXME change this
	fn get_size(&self) -> [i32;2] {
		return [0,0];
	}

	fn position(&mut self,x:i32,y:i32) {
		
	}
}