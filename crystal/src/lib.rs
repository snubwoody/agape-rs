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
}


