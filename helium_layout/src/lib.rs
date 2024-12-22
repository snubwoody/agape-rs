use std::{f32::INFINITY, path::Iter};

use helium_core::size::Size;

#[derive(Debug)]
pub struct LayoutSolver{
	root:LayoutNode
}

impl LayoutSolver {
	pub fn new(root:LayoutNode) -> Self{
		Self{root}
	}

	/// Calculates the layout of all the layout nodes
	pub fn solve(&mut self,window_size:Size){
		// Set the max constraints of the root node to the window size
		self.root.constraints.max_width = window_size.width;
		self.root.constraints.max_height = window_size.height;

		self.root.solve_max_contraints(window_size);
		self.root.update_size();
	
	}
}


#[derive(Debug,Default,Clone)]
pub struct LayoutNode{
	size:Size,
	intrinsic_size:IntrinsicSize,
	constraints:BoxContraints,
	children:Vec<Box<LayoutNode>>
}

impl LayoutNode {
	pub fn new() -> Self{
		Self::default()
	}

	pub fn add_child(&mut self,child:LayoutNode){
		self.children.push(Box::new(child));
	}

	fn solve_max_contraints(&mut self,space:Size) {
		// Sum up all the flex factors
		let flex_width_total:u8 = 
			self
			.children
			.iter()
			.filter_map(|child|{
				if let BoxSizing::Flex(factor) = child.intrinsic_size.width  {
					Some(factor)				
				}else {
					None
				}
			})
			.sum();
		
		let flex_height_total:u8 = 
			self
			.children
			.iter()
			.filter_map(|child|{
				if let BoxSizing::Flex(factor) = child.intrinsic_size.height  {
					Some(factor)				
				}else {
					None
				}
			})
			.sum();
		
		for child in &mut self.children{
			let mut max_size = Size::default();
			match child.intrinsic_size.width {
				BoxSizing::Flex(factor) => {
					// Make sure the factor isn't bigger than available size
					let grow_factor = 
						factor as f32 / flex_width_total as f32;
					
					max_size.width = grow_factor * space.width;
					child.constraints.max_width = max_size.width;
					
					// TODO replace with custom err 
					assert_ne!(grow_factor,INFINITY);
					
				}
				_ => {}
			}
			match child.intrinsic_size.height {
				BoxSizing::Flex(factor) => {
					let grow_factor = 
						factor as f32 / flex_height_total as f32;

					max_size.height = grow_factor * space.height;
					child.constraints.max_height = max_size.height;
					
					assert_ne!(grow_factor,INFINITY);					
				},
				_ => {}
			}

			// Pass the max size to the children to solve their max constraints
			child.solve_max_contraints(max_size);
		}
	}

	/// Update the size of every [`LayoutNode`] based on it's size and it's constraints
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

		for child in &mut self.children{
			child.update_size();
		}
	}


}

#[derive(Debug,Clone, Copy,PartialEq, Eq,Default)]
pub enum LayoutType {
	#[default]
	Block,
	Horizontal,
	Vertical,
}

#[derive(Debug,Clone, Copy,Default,PartialEq)]
pub enum BoxSizing{
	Fixed(f32),
	/// Tries to be as big as possible
	Flex(u8),
	#[default]
	/// Tries to be as small as possible
	Shrink,
}

#[derive(Debug,Clone, Copy,Default)]
pub struct BoxContraints{
	pub max_width:f32,
	pub max_height:f32,
	pub min_height:f32,
	pub min_width:f32
}

impl BoxContraints {
	pub fn new() -> Self{
		Self::default()
	}
}

/// This is the size that a [`Widget`] will try to be,  
/// the actual final size is dependant on the space
/// available.
#[derive(Debug,Clone,Copy,Default)]
pub struct IntrinsicSize {
	pub width:BoxSizing,
	pub height:BoxSizing
}

#[cfg(test)]
mod test{
	use super::*;

	#[test]
	fn test_grow_sizing(){
		let window = Size::new(800.0, 400.0);
		let mut node = LayoutNode::new();
		let mut inner_node = LayoutNode::new();
		
		node.intrinsic_size.width = BoxSizing::Flex(1);
		node.intrinsic_size.height = BoxSizing::Flex(1);
		
		inner_node.intrinsic_size.width = BoxSizing::Flex(1);
		inner_node.intrinsic_size.height = BoxSizing::Flex(2);

		node.add_child(inner_node.clone());
		node.add_child(inner_node);

		let mut solver = LayoutSolver::new(node);
		solver.solve(window);


		assert_eq!(solver.root.size,window);
		
		let half_size = window / 2.0;
	
		// The two children should both be half the size
		assert_eq!(solver.root.children[0].size,half_size);
		assert_eq!(solver.root.children[1].size,half_size);
	}

	#[test]
	fn test_inner_grow_sizing(){
		let window = Size::new(800.0, 400.0);
		let mut root_node = LayoutNode::new();
		let mut child_node = LayoutNode::new();
		let mut grand_child_node = LayoutNode::new();
		
		root_node.intrinsic_size.width = BoxSizing::Flex(1);
		root_node.intrinsic_size.height = BoxSizing::Flex(1);
		
		child_node.intrinsic_size.width = BoxSizing::Flex(1);
		child_node.intrinsic_size.height = BoxSizing::Flex(1);
		
		grand_child_node.intrinsic_size.width = BoxSizing::Flex(1);
		grand_child_node.intrinsic_size.height = BoxSizing::Flex(1);
		
		let mut first_node = child_node.clone();
		first_node.add_child(grand_child_node.clone());
		first_node.add_child(grand_child_node.clone());
		first_node.add_child(grand_child_node.clone());

		root_node.add_child(first_node);
		root_node.add_child(child_node);

		let mut solver = LayoutSolver::new(root_node);
		solver.solve(window);


		assert_eq!(solver.root.size,window);
		
		let half_size = window * 1.0/2.0;
		let qtr_size = half_size * 1.0/3.0;
	
		// The two children should both be half the size
		// TODO maybe add three for good measure
		assert_eq!(solver.root.children[0].size,half_size);
		assert_eq!(solver.root.children[1].size,half_size);
		
		// The two inner children should both be a third the half_size size
		// Round the sizes since floats are imprecise
		assert_eq!(
			solver.root.children[0].children[0].size.width.round(),
			qtr_size.width.round()
		);
		assert_eq!(
			solver.root.children[0].children[0].size.height.round(),
			qtr_size.height.round()
		);
		assert!(
			solver.root.children[0].children[0].size == 
			solver.root.children[0].children[1].size  
		);
	}

	#[test]
	fn test_max_sizing(){
		todo!()
		// TODO test the max sizing of a node and it's children
	}

	#[test]
	fn test_hybrid_layouts(){
		todo!()
	}

	#[test]
	fn test_shrink_sizing(){
		todo!()
	}
}


