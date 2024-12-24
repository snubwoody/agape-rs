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

pub struct LayoutSolver;

impl LayoutSolver {
	/// Calculates the layout of all the layout nodes
	pub fn solve(root:&mut dyn Layout,window_size:Size){
		// Set the max constraints of the root node to the window size
		root.set_max_width(window_size.width);
		root.set_max_height(window_size.height);

		root.solve_max_contraints(window_size);
		let _ = root.solve_min_constraints();
		root.update_size();
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


	fn id(&self) -> &str;
	fn constraints(&self) -> BoxContraints;
	fn intrinsic_size(&self) -> IntrinsicSize;
	fn size(&self) -> Size;
	fn position(&self) -> Position;
	fn children(&self) -> &[Box<dyn Layout>];

	fn set_max_width(&mut self,width:f32);
	fn set_max_height(&mut self,height:f32);
	fn set_min_width(&mut self,width:f32);
	fn set_min_height(&mut self,height:f32);
	fn set_position(&mut self,position:Position);
	fn set_x(&mut self,x:f32);
	fn set_y(&mut self,y:f32);

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
}


