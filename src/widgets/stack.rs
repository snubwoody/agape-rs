use std::marker;

use glium::{
	glutin::surface::WindowSurface, Display, Frame, Program, 
};
use winit::window::Window;
use crate::widgets::{Widget,rect::Rect};



/// A widget that arranges children in a vertical list 
pub struct VStack<'a>{
	pub spacing:i32,
	pub children:Vec<&'a mut Rect>
}


//TODO there might be unnecessary mutability here
impl<'a> VStack<'a> {
	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		program:&Program,
	){
		let mut offset = 0;
		self.children.iter_mut().for_each(|child|{
			let y_position = offset;
			child.set_position(0, y_position);
			child.render(display, frame, window, program);
			offset += self.spacing + child.height;
		});
	}
}

#[macro_export]
/// Creates an [`VStack`]
macro_rules! vstack {
	(
		spacing:$spacing:expr, 
		$($x:expr),
		*
	) => {
		{
			
            let mut children = Vec::new();
            $(
                children.push(&mut $x);
            )*

            VStack{
				spacing:$spacing,
				children:children
			}
        }
	};
}


pub struct HStack{
	pub x:i32,
	pub y:i32,
	pub width:i32,
	pub height:i32,
	pub spacing:i32,
	pub children:Vec<Box<dyn Widget>>
}


//TODO there might be unnecessary mutability here
impl Widget for HStack {
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		program:&Program,
	){
		let mut offset = 0;
		self.children.iter_mut().for_each(|child|{
			let y_position = offset;
			child.set_position(0, y_position);
			child.render(display, frame, window, program);
			let size = child.size();
			offset += self.spacing + size[0];
		});
	}

	fn set_position(&mut self,x:i32,y:i32) {
		self.x = x;
		self.y = y;
	}

	fn size(&mut self) -> [i32;2] {
		return [self.width,self.height];
	}
}

#[macro_export]
/// Creates an [`HStack`]
macro_rules! hstack {
	(
		spacing:$spacing:expr, 
		width:$width:expr,
		height:$height:expr,
		$($x:expr),
		*
	) => {
		{
			
            let mut children = Vec::new();
            $(
                children.push(Box::new($x))
            )*
            HStack{
				x:0,
				y:0,
				width:$width,
				height:$height,
				spacing:$spacing,
				children:children
			}
        }
	};
}
