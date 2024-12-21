use std::fmt::Debug;
use helium_core::{position::Position, size::Size};

#[derive(Debug)]
pub struct LayoutSolver{
	root:LayoutNode
}

impl LayoutSolver {
	pub fn new(root:LayoutNode) -> Self{
		Self{root}
	}
}


#[derive(Debug,Default)]
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

	pub fn add_child(mut self,child:LayoutNode) -> Self{
		self.children.push(Box::new(child));
		self
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
	Grow,
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
	fn test_layout(){
		let node = LayoutNode::new();
		let solver = LayoutSolver::new(node);
		dbg!(solver);
	}
}


