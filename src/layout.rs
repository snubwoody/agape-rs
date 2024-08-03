use crate::widgets::Widget;

//The different types of layout a widget can have
pub struct Horizontal;
pub struct Vertical;
pub struct Single;


// TODO implement padding
/// This struct handles the layout of widgets
pub struct Layout<L>{
	pub spacing:u32,
	pub padding:u32,
	pub layout:L
}

impl<L> Layout<L>  {
	pub fn new(spacing:u32,padding:u32,layout:L) -> Self{
		Self { spacing, padding, layout }
	}
}

impl Layout<Vertical> {
	pub fn arrange(&mut self,position:[u32;2],children:&mut Vec<Box<dyn Widget>>) -> (u32,u32) {
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

		let mut current_pos = position[1] + self.padding;

		children.iter_mut().for_each(|child|{
			let size = child.get_size();
			child.position(position[0] as i32 + self.padding as i32, current_pos as i32);
			current_pos += self.spacing + size.1;
		});

		max_width += self.padding *2;
		max_height += self.padding *2;

		(max_width,max_height)
	}
}

impl Layout<Horizontal> {
	pub fn arrange(&mut self,position:[u32;2],children:&mut Vec<Box<dyn Widget>>) -> (u32,u32) {
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

		let mut current_pos = position[0] + self.padding;
		children.iter_mut().for_each(|child|{
			let size = child.get_size();
			child.position(current_pos as i32, position[1] as i32 + self.padding as i32);
			current_pos += self.spacing + size.0;
		});

		max_width += self.padding *2;
		max_height += self.padding *2;

		(max_width,max_height)
	}
}

impl Layout<Single> {
	pub fn arrange(&mut self,position:[u32;2],child:&mut dyn Widget) -> (u32,u32) {
		let mut max_width = 0;
		let mut max_height = 0;

		// Position the child in the center of parent widget
		child.position(position[0] as i32 + self.padding as i32,position[1] as i32 + self.padding as i32);

		let child_size = child.get_size();

		max_width += child_size.0 + self.padding * 2;
		max_height += child_size.1 + self.padding * 2;

		(max_width,max_height)
	}
}

