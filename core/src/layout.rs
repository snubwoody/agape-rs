use crate::widgets::{Drawable, Widget, WidgetBody};

/// The types of layout a widget can have
#[derive(Debug)]
pub enum Layout{
	Horizontal{
		spacing:u32,
		padding:u32,
	},
	Vertical{
		spacing:u32,
		padding:u32,
	},
	Single{
		padding:u32,
	},
	SingleChild{
		width:u32,
		height:u32
	}
}

impl Layout {
	pub fn arrange(&self,position:[i32;2],children:&mut Vec<Box<WidgetBody>>) -> (u32,u32) {
		match self {
			&Self::Single { padding } => self.arrange_single(position,&mut children[0],padding),
			&Self::Vertical { spacing,padding } => self.arrange_vertical(),
			&Self::Horizontal { spacing,padding } => self.arrange_horizontal(),
			&Self::SingleChild { width,height} => {(width,height)}
		}
	}

	fn arrange_single(&self,position:[i32;2],child:&mut WidgetBody,padding:u32) -> (u32,u32) {
		let mut max_width = 0;
		let mut max_height = 0;
		

		// Position the child in the center of parent widget
		child.position(position[0] + padding as i32,position[1] + padding as i32);

		let child_size = child.get_size();

		max_width += child_size.0 + padding * 2;
		max_height += child_size.1 + padding * 2;

		(max_width,max_height) 
	}

	fn arrange_vertical(&self) -> (u32,u32) {
		/* let mut max_width = 0;
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

		let mut current_pos = position[1] + self.padding;

		children.iter_mut().for_each(|child|{
			let size = child.get_size();
			child.position(position[0] as i32 + self.padding as i32, current_pos as i32);
			current_pos += self.spacing + size.1;
		});

		max_width += self.padding *2;
		max_height += self.padding *2;

		(max_width,max_height) */
		(0,0)
	}

	fn arrange_horizontal(&self) -> (u32,u32) {
		/* let mut max_width = 0;
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

		let mut current_pos = position[0] + self.padding;
		children.iter_mut().for_each(|child|{
			let size = child.get_size();
			child.position(current_pos as i32, position[1] as i32 + self.padding as i32);
			current_pos += self.spacing + size.0;
		});

		max_width += self.padding *2;
		max_height += self.padding *2;

		(max_width,max_height) */

		(0,0)
	}
}

