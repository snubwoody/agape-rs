use crate::widgets::Widget;
use crate::surface::Surface;


/// A container [`Widget`] that can only have one child
pub struct Container<T>{
	surface:Surface,
	padding:i32, 
	child:T
}

impl<T> Container<T> where T:Widget {
	pub fn new(width:i32,height:i32,padding:i32,colour:[f32;4],child:T) -> Self{
		let surface = Surface::new(0, 0, width, height, colour);
		Self {
			surface,
			padding,
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
		program:&glium::Program,
	) {
		self.child.set_position(self.surface.x, self.surface.y);
		self.child.render(display, frame, window, program);
		self.surface.render(display, frame, window, program);
	}

	//FIXME change this
	fn size(&mut self) -> [i32;2] {
		return [0,0] ;	
	}

	fn set_position(&mut self,x:i32,y:i32) {
		self.surface.x = x;
		self.surface.y = y;
	}

}

// Do i need the children do be mutable or not? And should i have a reference or ownership?
