pub mod rect;
pub mod stack;
pub mod container;
pub mod text;
use glium::{
	glutin::surface::WindowSurface, Display, Frame, 
};
use winit::window::Window;
use crate::view::RenderContext;

/// Widget trait that all widgets must inherit from
pub trait Widget {
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	);
	
	/// Set the position of the [`Widget`]  
	/// Note that positions start from the upper left 
	/// corner
	fn position(&mut self,x:i32,y:i32);	
	
	/// Set the size of the widget
	fn size(&mut self,width:u32,height:u32);

	/// Get the size of the widget
	fn get_size(&self) -> (u32,u32);

}

/// Represents the sizing constraints a widget should have ie.
/// `Fit` the children, `Fill` the parent, `Relative` to the
/// parent as a percentage or a `Fixed` size in pixels
#[derive(Debug,Clone,Copy,PartialEq)] 
pub enum SizeContraint{
	Fill,
	Fit,
	Relative(f32,f32),
	Fixed(u32,u32)
}

pub trait Layout {
	fn arrange(&mut self,position:[u32;2],children:&mut Vec<Box<dyn Widget>>) -> (u32,u32);
}

struct VerticalLayout{
	spacing:u32
}

impl VerticalLayout {
	pub fn new(spacing:u32) -> Self{
		Self { spacing }
	}
}

impl Layout for VerticalLayout {
	fn arrange(&mut self,position:[u32;2],children:&mut Vec<Box<dyn Widget>>) -> (u32,u32) {
		let mut max_width = 0;
		let mut max_height = 0;
		// Iterate over the children to get the required space
		for (index,child) in children.iter().enumerate(){
			let (width,height) = child.get_size();
			if width > max_width{
				max_width = width
			}

			max_height += height;
			
			// Add the spacing for all elements except the last
			if index != children.len() - 1 {
				max_height += self.spacing;
			}
		};

		let mut current_pos = position[1];
		children.iter_mut().for_each(|child|{
			let size = child.get_size();
			child.position(position[0] as i32, current_pos as i32);
			current_pos += self.spacing + size.1;
		});

		(max_width,max_height)
	}
}
struct HorizontalLayout{
	spacing:u32
}

impl HorizontalLayout {
	pub fn new(spacing:u32) -> Self{
		Self { spacing }
	}
}

impl Layout for HorizontalLayout {
	fn arrange(&mut self,position:[u32;2],children:&mut Vec<Box<dyn Widget>>) -> (u32,u32) {
		let mut max_width = 0;
		let mut max_height = 0;
		
		// Iterate over the children to get the required space
		for (index,child) in children.iter().enumerate(){
			let (width,height) = child.get_size();
			if height > max_height{
				max_height = height
			}

			max_width += width;
			
			// Add the spacing for all elements except the last
			if index != children.len() - 1 {
				max_width += self.spacing;
			}
		};

		let mut current_pos = position[0];
		children.iter_mut().for_each(|child|{
			let size = child.get_size();
			child.position(current_pos as i32, position[1] as i32);
			current_pos += self.spacing + size.0;
		});

		(max_width,max_height)
	}
}
