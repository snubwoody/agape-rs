use properties::Drawable;
use crate::{
	colour::Colour, 
	layout::{Layout,Single}, 
	surface::Surface, 
	app::view::RenderContext, 
	widgets::Widget
};
use super::Drawable;

/// A container [`Widget`] that can only have one child
#[derive(Debug,Clone,Copy)]
#[derive(Drawable)]
pub struct Container<W:Widget>{
	surface:Surface,
	layout:Layout<Single>,
	child:W
}

impl<W> Container<W>
where W:Widget + Drawable
{
	pub fn new(child:W) -> Self{
		let surface = Surface::new(0, 0, 0, 0, Colour::Rgb(255, 255, 255));
		let layout = Layout::new(0, 64, Single);

		Self {
			surface,
			layout,
			child
		}
	}
}

impl<W> Widget for Container<W>
where W:Widget + Drawable + 'static {
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

	fn get_children(self) -> Vec<Box<dyn Widget>> {
		vec![Box::new(self.child)]
	}
}





