use properties::Position;
use crate::{
	colour::rgb, layout::{Layout, Single}, surface::Surface, view::RenderContext, widgets::Widget
};
use super::Drawable;

/// A container [`Widget`] that can only have one child
#[derive(Debug,Clone,Copy)]
#[derive(Position)]
pub struct Container<W:Widget>{
	surface:Surface,
	layout:Layout<Single>,
	child:W
}

impl<W> Container<W>
where W:Widget + Drawable
{
	pub fn new(child:W) -> Self{
		let surface = Surface::new(0, 0, 0, 0, rgb(255, 25, 255));
		let layout = Layout::new(0, 64, Single);

		Self {
			surface,
			layout,
			child
		}
	}
}

impl<W> Widget for Container<W>
where W:Widget + Drawable {
	fn render(
		&mut self,
		display:&glium::Display<glium::glutin::surface::WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		context:&RenderContext
	) {
		let position = [self.surface.x as u32,self.surface.y as u32];
		let (width,height) = self.layout.arrange(position, &mut self.child);
		self.size(width, height);

		self.surface.render(display, frame, window, &context.surface_program);
		self.child.render(display, frame, window,context);
	}

	/* fn size(&mut self,width:u32,height:u32) {
		self.surface.width = width as i32;
		self.surface.height = height as i32;
	}

	fn get_size(&self) -> (u32,u32) {
		(self.surface.width as u32,self.surface.height as u32)
	} */

	fn arrange_widgets(&mut self) {
		//Empty
	}
}




