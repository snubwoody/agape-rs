use std::marker;

use glium::{
	glutin::surface::WindowSurface, Display, Frame, Program, 
};
use winit::window::Window;
use crate::{surface::Surface, widgets::Widget};


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
		program:&Program,
	){
		self.surface.render(display, frame, window, program);
		let mut offset = 0;
		self.children.iter_mut().for_each(|child|{
			let position = offset;
			child.render(display, frame, window, program);

			// TODO might cause issues due to setting the other position to 0
			// Try setting the position of the child during initialization
			// Then try translating instead
			match self.direction {
				StackDirection::Horizontal => {
					let size = child.get_size();
					offset += self.spacing + size[0];
					child.position(position, 0);
				},
				StackDirection::Vertical => {
					let size = child.get_size();
					offset += self.spacing + size[1];
					child.position(0, position);
				}
			}
		});
	}

	fn position(&mut self,x:i32,y:i32) {
		self.surface.x = x;
		self.surface.y = y;
	}

	fn get_size(&mut self) -> [i32;2] {
		return [self.surface.width,self.surface.height];
	}
}



#[macro_export]
/// A Phantom [`Widget`] that returns a stack with a horizontal direction
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
                children.push(Box::new($x) as Box<dyn Widget>);
            )*

            let surface = Surface::new(0, 0, $width, $height, rgb(255,255,255));

            Stack{
				surface,
				direction:StackDirection::Horizontal,
				spacing:$spacing,
				children:children
			}
        }
	};
}

#[macro_export]
/// A Phantom [`Widget`] that returns a stack with a vertical direction
macro_rules! vstack {
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
                children.push(Box::new($x) as Box<dyn Widget>);
            )*

			let surface = Surface::new(0, 0, $width, $height, rgb(255,255,255));

            Stack{
				surface,
				direction:StackDirection::Vertical,
				spacing:$spacing,
				children:children
			}
        }
	};
}

