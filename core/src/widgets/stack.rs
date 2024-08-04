use glium::{
	glutin::surface::WindowSurface, Display, Frame,  
};
use properties::Drawable;
use winit::window::Window;
use crate::colour::Colour;
use crate::{surface::Surface, view::RenderContext, widgets::Widget};
use crate::layout::{Horizontal, Layout, Vertical};

use super::Drawable;

#[derive(Drawable)]
pub struct VStack{
	surface:Surface,
	layout:Layout<Vertical>,
	children:Vec<Box<dyn Widget>>
}

impl VStack {
	pub fn new(spacing:u32,children:Vec<Box<dyn Widget>>) -> Self{
		let surface = Surface::new(0, 0, 0, 0, Colour::Rgb(255, 255, 255));
		let layout = Layout::new(spacing, 120, Vertical);

		Self { surface, children,layout }
	}

	fn arrange_widgets(&mut self){
		let (x,y) = (self.surface.x as u32,self.surface.y as u32);
		let (max_width,max_height) = self.layout.arrange([x,y], &mut self.children);
		self.size(max_width,max_height);
	}
}

impl Widget for VStack {
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		// FIXME cannot properly size children because the parent is rendered before the child
		let position = [self.surface.x as u32,self.surface.y as u32];
		//FIXME 
		//let (width,height) = self.layout.arrange(position, &mut self.children);
		//self.size(width, height);

		self.surface.render(display, frame, window, &context.surface_program);
		self.children.iter_mut().for_each(|child|{
			child.render(display, frame, window, context)
		});
	}

	
}


#[derive(Drawable)]
pub struct HStack{
	surface:Surface,
	layout:Layout<Horizontal>,
	children:Vec<Box<dyn Widget>>
}

impl HStack {
	pub fn new(spacing:u32,children:Vec<Box<dyn Widget>>) -> Self{
		let surface = Surface::new(0, 0, 0, 0, Colour::Rgb(255, 255, 255));
		let layout = Layout::new(spacing, 0, Horizontal);
		Self { surface,layout,children }
	}

	fn arrange_widgets(&mut self) {
		let (x,y) = (self.surface.x as u32,self.surface.y as u32);
		let (max_width,max_height) = self.layout.arrange([x,y], &mut self.children);
		self.size(max_width,max_height);
	}
}

impl Widget for HStack {
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		self.arrange_widgets();
		self.surface.render(display, frame, window, &context.surface_program);
		self.children.iter_mut().for_each(|child|{
			child.render(display, frame, window, context)
		})
	}

	
}
