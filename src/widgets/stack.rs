use glium::{
	glutin::surface::WindowSurface, Display, Frame,  
};
use winit::window::Window;
use crate::{colour::rgb, surface::{self, Surface}, view::RenderContext, widgets::Widget};
use super::SizeContraint;


pub enum StackDirection {
	Horizontal,
	Vertical
}

/// A [`Widget`] that arranges it's children either
/// horizontally or vertically.
pub struct Stack{
	pub surface:Surface,
	pub spacing:i32,
	pub direction:StackDirection,
	pub children:Vec<Box<dyn Widget>>
}

impl Widget for Stack {
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		self.surface.render(display, frame, window, &context.surface_program);
		let mut offset = 0;
		
		self.children.iter_mut().for_each(|child|{
			let position = offset;
			child.render(display, frame, window, &context);

			// TODO might cause issues due to setting the other position to 0
			// Try setting the position of the child during initialization
			// Then try translating instead
			match self.direction {
				StackDirection::Horizontal => {
					let (width,hight) = child.get_size();
					offset += self.spacing + width as i32;
					child.position(position, 0);
				},
				StackDirection::Vertical => {
					let (width,height) = child.get_size();
					offset += self.spacing + height as i32;
					child.position(0, position);
				}
			}
		});
	}

	fn position(&mut self,x:i32,y:i32) {
		self.surface.x = x;
		self.surface.y = y;
	}

	fn size(&mut self,width:u32,height:u32) {
		self.surface.width = width as i32;
		self.surface.height = height as i32;
	}

	fn get_size(&self) -> (u32,u32) {
		(self.surface.width as u32,self.surface.height as u32)
	}

	fn arrange_widgets(&mut self,max_size:[u32;2]) {
		
	}

}

pub struct VStack{
	surface:Surface,
	spacing:u32,
	children:Vec<Box<dyn Widget>>
}

impl VStack {
	pub fn new(spacing:u32,children:Vec<Box<dyn Widget>>) -> Self{
		let surface = Surface::new(0, 0, 0, 0, rgb(255, 255, 255), SizeContraint::Fit);
		Self { surface, spacing, children }
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
		self.surface.render(display, frame, window, &context.surface_program);
		self.children.iter_mut().for_each(|child|{
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
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		self.surface.render(display, frame, window, &context.surface_program);
		self.children.iter_mut().for_each(|child|{
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
