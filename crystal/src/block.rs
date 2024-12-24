use std::f32::INFINITY;
use helium_core::{position::Position, size::Size};
use crate::{BoxContraints, BoxSizing, IntrinsicSize, Layout};

/// This layout only has one child
pub struct BlockLayout{ // TODO add padding
	pub id:String,
	size:Size,
	position:Position,
	pub padding:u32,
	pub intrinsic_size:IntrinsicSize,
	// TODO i'm thinking of adding user constraints as well so that people can define their own 
	// constraints
	constraints:BoxContraints,
	pub child:Box<dyn Layout>
}

impl BlockLayout {
	pub fn new(child:Box<dyn Layout>) -> Self{
		Self{
			id:String::default(),
			size:Size::default(),
			padding:0,
			position:Position::default(),
			intrinsic_size:IntrinsicSize::default(),
			constraints:BoxContraints::default(),
			child
		}
	}

	fn fixed_size_sum(&self) -> Size{
		let mut sum = Size::default();

		match self.child.intrinsic_size().width {
			BoxSizing::Fixed(width) => {
				sum.width = sum.width.max(width);
			},
			_ => {}
		}
		
		match self.child.intrinsic_size().height {
			BoxSizing::Fixed(height) => {
				sum.height += height;
			},
			_ => {}
		}

		sum
	}

}


impl Layout for BlockLayout {
	fn id(&self) -> &str {
		&self.id
	}

	fn size(&self) -> Size {
		self.size
	}

	
	fn set_position(&mut self,position:Position) {
		self.position = position;
	}

	fn set_x(&mut self,x:f32) {
		self.position.x = x;
	}

	fn set_y(&mut self,y:f32) {
		self.position.y = y;
	}

	fn position(&self) -> Position {
		self.position
	}

	fn children(&self) -> &[Box<dyn Layout>] {
		std::slice::from_ref(&self.child)
	}

	fn constraints(&self) -> BoxContraints {
		self.constraints
	}

	fn intrinsic_size(&self) -> IntrinsicSize {
		self.intrinsic_size
	}

	fn set_max_height(&mut self,height:f32) {
		self.constraints.max_height = height;
	}
	
	fn set_max_width(&mut self,width:f32) {
		self.constraints.max_width = width;
	}
	
	fn set_min_height(&mut self,height:f32) {
		self.constraints.min_height = height;
	}
	
	fn set_min_width(&mut self,width:f32) {
		self.constraints.min_width = width;
	}

	fn solve_min_constraints(&mut self) -> (f32,f32){
		// The sum of the size of all the children with fixed sizes
		let fixed_sum = self.fixed_size_sum();

		// TODO i think im supposed to calculate the min constraints of the children as well
		match self.intrinsic_size.width {
			BoxSizing::Fixed(width) => {
				self.constraints.min_width = width;	
			},
			BoxSizing::Flex(_) => {
				// TODO maybe set the min constraints to either 0 or the size of the children
			},
			BoxSizing::Shrink => {
				self.constraints.min_width = fixed_sum.width;	
			},
		}
		
		match self.intrinsic_size.height {
			BoxSizing::Fixed(height) => {
				self.constraints.min_height = height;	
			},
			BoxSizing::Flex(_) => {

			},
			BoxSizing::Shrink => {
				self.constraints.min_height = fixed_sum.height;	
			},
		}

		(self.constraints.min_width,self.constraints.min_height)
	}

	fn solve_max_contraints(&mut self,space:Size) {
		match self.child.intrinsic_size().width {
			BoxSizing::Flex(_) => {
				self.child.set_max_width(space.width);
			}
			BoxSizing::Fixed(width) => {
				self.child.set_max_width(width);
			}
			BoxSizing::Shrink => {}
		}
		
		match self.child.intrinsic_size().height {
			BoxSizing::Flex(_) => {
				self.child.set_max_height(space.height);					
			},
			BoxSizing::Fixed(height) => {
				self.child.set_max_height(height);
			}
			BoxSizing::Shrink => {}
		}

		// Pass the max size to the children to solve their max constraints
		self.child.solve_max_contraints(space);
	}

	

	fn update_size(&mut self){
		match self.intrinsic_size.width {
			BoxSizing::Flex(_) => {
				self.size.width = self.constraints.max_width;
			},
			BoxSizing::Shrink => {
				self.size.width = self.constraints.min_width;
			},
			BoxSizing::Fixed(width) => {
				// TODO maybe set the min constrains?
				self.size.width = width;
			}
		}

		match self.intrinsic_size.height {
			BoxSizing::Flex(_) => {
				self.size.height = self.constraints.max_height;
			},
			BoxSizing::Shrink => {
				self.size.height = self.constraints.min_height;
			},
			BoxSizing::Fixed(height) => {
				// TODO maybe set the min constrains?
				self.size.height = height;
			}
		}

		self.child.update_size();
	}

	fn position_children(&mut self){
		let mut current_pos = self.position;
		current_pos += self.padding as f32 * 2.0; 
		self.child.set_position(current_pos);
	}
}

#[cfg(test)]
mod test{
	use crate::{EmptyLayout, LayoutSolver};
	use super::*;

	#[test]
	fn test_shrink_sizing(){
		let window = Size::new(800.0, 800.0);
		let mut child = EmptyLayout::new();
		child.intrinsic_size.width = BoxSizing::Fixed(200.0);
		child.intrinsic_size.height = BoxSizing::Fixed(200.0);

		// TODO add padding
		let mut root = BlockLayout::new(Box::new(child)); 
		LayoutSolver::solve(&mut root, window);

		assert_eq!(root.size(),Size::new(200.0, 200.0));
	}

}