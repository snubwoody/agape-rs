use std::f32::INFINITY;
use helium_core::{position::Position, size::Size};
use crate::{AxisAlignment, BoxContraints, BoxSizing, IntrinsicSize, Layout, LayoutIter};

/// This layout only has one child
#[derive(Debug)]
pub struct BlockLayout{
	pub id:String,
	pub size:Size,
	pub position:Position,
	pub padding:u32,
	pub intrinsic_size:IntrinsicSize,
	// TODO i'm thinking of adding user constraints as well so that people can define their own 
	// constraints
	pub constraints:BoxContraints,
	/// The main axis is the `x-axis`
	pub main_axis_alignment:AxisAlignment,
	/// The main axis is the `y-axis`
	pub cross_axis_alignment:AxisAlignment,
	pub child:Box<dyn Layout>,
	pub errors:Vec<crate::LayoutError>
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
			main_axis_alignment:AxisAlignment::default(),
			cross_axis_alignment:AxisAlignment::default(),
			errors:vec![],
			child
		}
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

	fn sort_children(&mut self) {
		// self.child.sort_children();
	}

	
	fn collect_errors(&self) -> Vec<crate::LayoutError> {
		self.errors
		.iter()
		.cloned()
		.chain(
			self
			.child
			.iter()
			.flat_map(|child|child.collect_errors())
			.collect::<Vec<_>>()
		).collect::<Vec<_>>()
	}

	fn iter(&self) -> LayoutIter {
		LayoutIter{
			stack:vec![Box::new(self)]
		}
	}

	fn solve_min_constraints(&mut self) -> (f32,f32){
		// The sum of the size of all the children with fixed sizes
		let mut min_size = Size::default();
		min_size += self.padding as f32 * 2.0;

		// Solve the fix size first
		match self.intrinsic_size.width {
			BoxSizing::Fixed(width) => {
				self.constraints.min_width = width;	
			},
			_ => {}
		}
		
		match self.intrinsic_size.height {
			BoxSizing::Fixed(height) => {
				self.constraints.min_height = height;	
			},
			_ => {},
		}

		
		match self.child.intrinsic_size().width {
			BoxSizing::Fixed(width) => {
				self.constraints.min_width = width + self.padding as f32 * 2.0;
			},
			_ => {}
		}
		
		match self.child.intrinsic_size().height {
			BoxSizing::Fixed(height) => {
				self.constraints.min_height = height + self.padding as f32 * 2.0;
			},
			_ => {}
		}

		let (min_width,min_height) = self.child.solve_min_constraints();
		
		// TODO i think im supposed to calculate the min constraints of the children as well
		match self.intrinsic_size.width {
			BoxSizing::Flex(_) => {
				// TODO maybe set the min constraints to either 0 or the size of the children
				self.constraints.min_width = min_width + self.padding as f32 * 2.0;	
			},
			BoxSizing::Shrink => {
				self.constraints.min_width = min_width + self.padding as f32 * 2.0;	
			},
			_ => {}
		}
		
		match self.intrinsic_size.height {
			BoxSizing::Flex(_) => {
				self.constraints.min_height = min_height + self.padding as f32 * 2.0;	
			},
			BoxSizing::Shrink => {
				self.constraints.min_height = min_height + self.padding as f32 * 2.0;	
			},
			_ => {}
		}

		(self.constraints.min_width,self.constraints.min_height)
	}

	fn solve_max_contraints(&mut self,space:Size) {
		let available_space = space - self.padding as f32;

		match self.child.intrinsic_size().width {
			BoxSizing::Flex(_) => {
				self.child.set_max_width(available_space.width);
			}
			BoxSizing::Fixed(width) => {
				self.child.set_max_width(width);
			}
			BoxSizing::Shrink => {}
		}
		
		match self.child.intrinsic_size().height {
			BoxSizing::Flex(_) => {
				self.child.set_max_height(available_space.height);					
			},
			BoxSizing::Fixed(height) => {
				self.child.set_max_height(height);
			}
			BoxSizing::Shrink => {}
		}

		// Pass the max size to the children to solve their max constraints
		self.child.solve_max_contraints(available_space);
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
		current_pos += self.padding as f32; 
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
		root.padding = 24; 
		LayoutSolver::solve(&mut root, window);

		assert_eq!(root.size(),Size::new(200.0 + 24.0 * 2.0, 200.0 + 24.0 * 2.0));
	}

	#[test]
	fn test_nested_shrink(){
		let window = Size::new(800.0, 800.0);

		let mut inner_child = EmptyLayout::new();
		inner_child.intrinsic_size.width = BoxSizing::Fixed(175.0);
		inner_child.intrinsic_size.height = BoxSizing::Fixed(15.0);
		
		let mut child = BlockLayout::new(Box::new(inner_child));
		child.padding = 24;
		
		let mut root = BlockLayout::new(Box::new(child)); 

		LayoutSolver::solve(&mut root, window);

		let inner_size = Size::new(175.0, 15.0);
		let child_size = inner_size + 24.0 * 2.0;

		assert_eq!(
			root.size(),
			child_size
		);
		assert_eq!(
			root.child.size(),
			child_size
		);
		assert_eq!(
			root.child.children()[0].size(),
			inner_size
		);
	}
	
	#[test]
	fn test_grow(){
		let window = Size::new(800.0, 800.0);
		let mut child = EmptyLayout::new();
		child.intrinsic_size.width = BoxSizing::Flex(1);
		child.intrinsic_size.height = BoxSizing::Flex(1);

		// TODO add padding
		let mut root = BlockLayout::new(Box::new(child));
		root.intrinsic_size.width = BoxSizing::Flex(1);
		root.intrinsic_size.height = BoxSizing::Flex(1);
		root.padding = 24; 
		
		LayoutSolver::solve(&mut root, window);

		let child_size = window - root.padding as f32;
		assert_eq!(
			root.size(),
			window
		);
		assert_eq!(
			root.child.size(),
			child_size
		);
	}

	#[test]
	fn test_inner_grow(){
		let window = Size::new(800.0, 800.0);
		let mut inner_child = EmptyLayout::new();
		inner_child.intrinsic_size.width = BoxSizing::Flex(1);
		inner_child.intrinsic_size.height = BoxSizing::Flex(1);
		
		let mut child = BlockLayout::new(Box::new(inner_child));
		child.intrinsic_size.width = BoxSizing::Flex(1);
		child.intrinsic_size.height = BoxSizing::Flex(1);
		
		// TODO add padding
		let mut root = BlockLayout::new(Box::new(child));
		root.intrinsic_size.width = BoxSizing::Flex(1);
		root.intrinsic_size.height = BoxSizing::Flex(1);
		root.padding = 24; 
		
		LayoutSolver::solve(&mut root, window);

		let child_size = window - root.padding as f32;
		assert_eq!(
			root.size(),
			window
		);
		assert_eq!(
			root.child.size(),
			child_size
		);
		assert_eq!(
			root.child.size(),
			root.child.children()[0].size()
		);
	}
}