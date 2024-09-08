use crate::widgets::WidgetBody;

/// The types of layout a [`Widget`] can have.
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
	},
}

impl Layout {
	pub fn arrange(&self,position:[f32;2],children:&mut Vec<Box<WidgetBody>>) -> (u32,u32) {
		match self {
			&Self::Single { padding } => self.arrange_single(position,&mut children[0],padding),
			&Self::Vertical { spacing,padding } => self.arrange_vertical(position,children,padding,spacing),
			&Self::Horizontal { spacing,padding } => self.arrange_horizontal(position,children,padding,spacing),
			&Self::SingleChild { width,height} => {(width,height)}
		}
	}

	fn arrange_single(&self,position:[f32;2],child:&mut WidgetBody,padding:u32) -> (u32,u32) {
		let mut max_width = 0;
		let mut max_height = 0;

		// Position the child in the center of parent widget
		child.surface.position(position[0] + padding as f32,position[1] + padding as f32);

		let child_size = child.surface.get_size();

		max_width += child_size.width as u32 + padding * 2;
		max_height += child_size.height as u32 + padding * 2;

		(max_width,max_height) 
	}

	fn arrange_vertical(&self,position:[f32;2],children:&mut Vec<Box<WidgetBody>>,padding:u32,spacing:u32) -> (u32,u32) {
		let mut max_width = 0;
		let mut max_height = 0;
		
		// Iterate over the children to get the required space
		for (index,child) in children.iter().enumerate(){
			let size = child.surface.get_size();
			if size.width as u32 > max_width{
				max_width = size.width as u32
			}

			max_height += size.height as u32;
			
			// Add the spacing for all elements except the last
			if index != children.len() - 1 {
				max_height += spacing;
			}
		};

		let mut current_pos = position[1] + padding as f32;

		children.iter_mut().for_each(|child|{
			let size = child.surface.get_size();
			child.surface.position(position[0] + padding as f32, current_pos);
			current_pos += spacing as f32 + size.height as f32;
		});

		max_width += padding *2;
		max_height += padding *2;

		(max_width,max_height)
	}

	fn arrange_horizontal(&self,position:[f32;2],children:&mut Vec<Box<WidgetBody>>,padding:u32,spacing:u32) -> (u32,u32) {
		let mut max_width = 0;
		let mut max_height = 0;
	
		// Iterate over the children to get the required space
		for (index,child) in children.iter().enumerate(){
			let size = child.surface.get_size();
			
			if size.height as u32 > max_height{
				max_height = size.height as u32
			}

			max_width += size.width as u32;
			
			// Add the spacing for all elements except the last
			if index != children.len() - 1 {
				max_width += spacing;
			}
		};

		// TODO decide whether to place objects at the baseline or at the top
		let mut current_pos = position[0] + padding as f32;
		children.iter_mut().for_each(|child|{
			let size = child.surface.get_size();
			child.surface.position(current_pos, position[1] + padding as f32);
			current_pos += spacing as f32 + size.width as f32;
		});

		max_width += padding *2;
		max_height += padding *2;

		(max_width,max_height)
	}
}


#[derive(Debug)]
pub enum SizeConstraint {
	Fill,
	Fit,
	Flex,
	Relative(f32)	
}
