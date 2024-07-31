use std::rc::Rc;

use glium::{
	glutin::surface::WindowSurface, Display, Frame,  
};
use winit::window::Window;
use crate::{colour::rgb, surface::{self, Surface}, view::RenderContext, widgets::Widget};
use super::{Layout, SizeContraint, VerticalLayout};

pub struct VStack{
	surface:Surface,
	spacing:u32,
	layout:VerticalLayout,
	children:Vec<Box<dyn Widget>>
}

impl VStack {
	pub fn new(spacing:u32,children:Vec<Box<dyn Widget>>) -> Self{
		let surface = Surface::new(0, 0, 0, 0, rgb(255, 255, 255), SizeContraint::Fit);
		let layout = VerticalLayout::new(spacing);
		Self { surface, spacing, children,layout }
	}

	pub fn colour(mut self,colour:[f32;4]) -> Self{
		self.surface.colour = colour;
		self
	}
}

impl Widget for VStack {
	fn render(
		&self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		//self.layout.arrange([800,800], self.children);
		self.surface.render(display, frame, window, &context.surface_program);
		self.children.iter().for_each(|child|{
			child.render(display, frame, window, context)
		})

	}

	fn position(&mut self,x:i32,y:i32) {
		self.surface.x = x;
		self.surface.y = y;
	}

	fn size(&mut self,width:u32,height:u32) {
		
	}

	fn get_size(&self) -> (u32,u32) {
		(self.surface.width as u32,self.surface.height as u32)
	}

	fn arrange_widgets(&mut self,max_size:[u32;2]) {
		let mut total_width = 0;
		let mut totol_height = 0;
		self.children.iter_mut().for_each(|child|{
			let (width,height) = child.get_size();
			total_width += width;
			totol_height += height + self.spacing;
		});

		dbg!(total_width,totol_height);
	}
}

pub struct HStack{
	surface:Surface,
	spacing:u32,
	children:Vec<Box<dyn Widget>>
}

impl HStack {
	pub fn new(spacing:u32,children:Vec<Box<dyn Widget>>) -> Self{
		let surface = Surface::new(0, 0, 0, 0, rgb(255, 255, 255), SizeContraint::Fit);
		Self { surface, spacing, children }
	}
}

impl Widget for HStack {
	fn render(
		&self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		self.surface.render(display, frame, window, &context.surface_program);
		self.children.iter().for_each(|child|{
			child.render(display, frame, window, context)
		})
	}

	fn position(&mut self,x:i32,y:i32) {
		self.surface.x = x;
		self.surface.y = y;
	}

	fn size(&mut self,width:u32,height:u32) {
		
	}

	fn get_size(&self) -> (u32,u32) {
		(self.surface.width as u32,self.surface.height as u32)
	}

	fn arrange_widgets(&mut self,max_size:[u32;2]) {
		
	}
}
