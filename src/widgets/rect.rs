use glium::{
	glutin::surface::WindowSurface, Display, Frame, 
};
use winit::window::Window;
use crate::{widgets::Widget,Surface};
use crate::layout::Layout;
use crate::view::RenderContext;


/// A simple rectangle
// TODO change this to use surface
pub struct Rect{
	surface:Surface
}

impl Rect {
	pub fn new(x:i32,y:i32,width:i32,height:i32,colour:[f32;4]) -> Self {
		
		Self{
			surface:Surface::new(x,y,width,height,colour)
		}
	}


}

//FIXME replace this with surface
impl Widget for Rect {
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		self.surface.render(display, frame, window, &context.surface_program);
	}

	fn position(&mut self,x:i32,y:i32){
		self.surface.x = x;
		self.surface.y = y;
	}

	fn get_size(&self) -> [i32;2] {
		return [self.surface.height,self.surface.height];
	}
}

impl Layout for Rect {
	fn arrange_widgets(&self){

	}
}
