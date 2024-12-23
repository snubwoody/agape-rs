//! This is the crate that manages all the helium layouts, at a basic level every layout
//! node must return a size and position so that other layouts can arrange themselves 
//! accordingly.
mod horizontal;
mod vertical;
mod block;
mod empty;
use std::f32::INFINITY;
use helium_core::{position::Position, size::Size};
pub use horizontal::HorizontalLayout;
pub use vertical::VerticalLayout;
pub use block::BlockLayout;
pub use empty::EmptyLayout;
pub struct LayoutSolver{
	root:Box<dyn Layout>
}

impl LayoutSolver {
	pub fn new(root:impl Layout + 'static) -> Self{
		Self{root:Box::new(root)}
	}

	/// Calculates the layout of all the layout nodes
	pub fn solve(&mut self,window_size:Size){
		// Set the max constraints of the root node to the window size
		self.root.set_max_width(window_size.width);
		self.root.set_max_height(window_size.height);

		self.root.solve_max_contraints(window_size);
		let _ = self.root.solve_min_constraints();
		self.root.update_size();
	}
}

pub trait Layout{
	/// Solve the minimum constraints of each layout node recursively, if 
	/// the node has an instrinsic size of `Fixed` then it's minimum size is 
	/// set to the fixed values, if it's intrinsic size is set to `Shrink` then
	/// it get's the min constraints of it's children bubbling them up the layout
	/// tree.
	fn solve_min_constraints(&mut self) -> (f32,f32);

	fn solve_max_contraints(&mut self,space:Size);
	
	/// Update the size of every [`LayoutNode`] based on it's size and constraints.
	fn update_size(&mut self);


	fn constraints(&self) -> BoxContraints;
	fn intrinsic_size(&self) -> IntrinsicSize;
	fn size(&self) -> Size;
	fn children(&self) -> &[Box<dyn Layout>];

	fn set_max_width(&mut self,width:f32);
	fn set_max_height(&mut self,height:f32);
	fn set_min_width(&mut self,width:f32);
	fn set_min_height(&mut self,height:f32);

}


#[derive(Debug,Default,Clone)]
pub struct LayoutNode{
	size:Size,
	position:Position,
	intrinsic_size:IntrinsicSize,
	// TODO i'm thinking of adding user constraints as well so that people can define their own 
	// constraints
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

	/// Calculate the sum of all the nodes with fixed sizes
	fn fixed_size_sum(&self) -> Size{
		let mut sum = Size::default();

		for child in &self.children{
			match child.intrinsic_size.width {
				BoxSizing::Fixed(width) => {
					sum.width += width;
				},
				_ => {}
			}

			match child.intrinsic_size.height {
				BoxSizing::Fixed(height) => {
					sum.height += height;
				},
				_ => {}
			}
		}

		sum
	}

	/// Solve the minimum constraints of each layout node recursively, if 
	/// the node has an instrinsic size of `Fixed` then it's minimum size is 
	/// set to the fixed values, if it's intrinsic size is set to `Shrink` then
	/// it get's the min constraints of it's children bubbling them up the layout
	/// tree.
	fn solve_min_constraints(&mut self) -> (f32,f32){
		// The sum of the size of all the children with fixed sizes
		let fixed_sum = self.fixed_size_sum();

		// TODO don't forget about the flex and shrink children
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
	/// Tries to be as big as possible, the behaviour of the flex factor is 
	/// dependant on the type of layout.
	Flex(u8),
	#[default]
	/// Tries to be as small as possible
	Shrink,
}

#[derive(Debug,Clone, Copy,Default,PartialEq)]
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
pub struct IntrinsicSize { // TODO does this really need to be a seperate struct?
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

		//let mut solver = LayoutSolver::new(node);
		//solver.solve(window);


		//assert_eq!(solver.root.size,window);
		
		let half_size = window / 2.0;
	
		// The two children should both be half the size
		//assert_eq!(solver.root.children[0].size,half_size);
		//assert_eq!(solver.root.children[1].size,half_size);
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

		todo!();

		// let mut solver = LayoutSolver::new(root_node);
		// solver.solve(window);


		// assert_eq!(solver.root.size,window);
		
		// let half_size = window * 1.0/2.0;
		// let inner_size = half_size * 1.0/3.0;
	
		// // The two children should both be half the size
		// assert_eq!(solver.root.children[0].size,half_size);
		// assert_eq!(solver.root.children[1].size,half_size);
		
		// // The two inner children should both be a third the half_size size
		// // Round the sizes since floats are imprecise
		// assert_eq!(
		// 	solver.root.children[0].children[0].size.width.round(),
		// 	inner_size.width.round()
		// );
		// assert_eq!(
		// 	solver.root.children[0].children[0].size.height.round(),
		// 	inner_size.height.round()
		// );
		// assert!(
		// 	solver.root.children[0].children[0].size == 
		// 	solver.root.children[0].children[1].size  
		// );
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
		let window = Size::new(800.0, 400.0);
		let mut root_node = LayoutNode::new();
		let mut child_node = LayoutNode::new();

		child_node.intrinsic_size.width = BoxSizing::Fixed(200.0);
		child_node.intrinsic_size.height = BoxSizing::Fixed(50.0);

		root_node.add_child(child_node.clone());
		root_node.add_child(child_node);

		// let mut solver = LayoutSolver::new(root_node);

		// solver.solve(window);

		// assert_eq!(solver.root.size,Size::new(400.0, 100.0));
		// assert_eq!(solver.root.children[0].size,Size::new(200.0, 50.0));
	}

	#[test]
	fn test_inner_shrink_sizing(){
		let window = Size::new(800.0, 800.0);

		let mut root_node = LayoutNode::new();
		let mut child_node = LayoutNode::new();
		let mut grand_child_node = LayoutNode::new();

		grand_child_node.intrinsic_size.width = BoxSizing::Fixed(50.0);
		grand_child_node.intrinsic_size.height = BoxSizing::Fixed(200.0);
		
		child_node.add_child(grand_child_node.clone());
		child_node.add_child(grand_child_node);
		
		root_node.add_child(child_node.clone());
		root_node.add_child(child_node);

		// let mut solver = LayoutSolver::new(root_node);

		// solver.solve(window);

		// // There's two 'shrink' children in the root and two 'fixed' children
		// // in each of the two children 
		// assert_eq!(solver.root.size,Size::new(200.0, 400.0));
		// assert_eq!(
		// 	solver.root.children[0].size,
		// 	Size::new(100.0, 400.0)
		// );
	}
}


