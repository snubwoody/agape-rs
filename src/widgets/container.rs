use crate::widgets::Widget;
use crate::surface::Surface;
use crate::view::RenderContext;


/// A container [`Widget`] that can only have one child
pub struct Container<T>{
	surface:Surface,
	child:T
}

impl<T> Container<T> where T:Widget {
	pub fn new(width:i32,height:i32,colour:[f32;4],child:T) -> Self{
		let surface = Surface::new(0, 0, width, height, colour);
		Self {
			surface,
			child:child
		}
	}
}

impl<T> Widget for Container<T> where T:Widget {
	fn render(
		&mut self,
		display:&glium::Display<glium::glutin::surface::WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		context:&RenderContext
	) {
		self.surface.render(display, frame, window, &context.surface_program);
		self.child.position(self.surface.x, self.surface.y);
		self.child.render(display, frame, window,context);
	}

	fn get_size(&mut self) -> [i32;2] {
		let width = self.surface.width;
		let height = self.surface.height;
		return [width,height] ;	
	}

	fn position(&mut self,x:i32,y:i32) {
		self.surface.x = x;
		self.surface.y = y;
	}

}

// Do i need the children do be mutable or not? And should i have a reference or ownership?
