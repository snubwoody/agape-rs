use std::f32::INFINITY;
use helium_core::{position::Position, size::Size};
use crate::{BoxContraints, BoxSizing, IntrinsicSize, Layout};

/// A [`HorizontalLayout`] sizes and position it's children horizontally, of course, the `Flex` 
/// attribute mean a layout node will fill it's widget, however the flex factor only works in 
/// the x-axis, in the y-axis all nodes will fill the parent and will be the same height.
#[derive(Default)]
pub struct HorizontalLayout{ // TODO add padding
	size:Size,
	position:Position,
	intrinsic_size:IntrinsicSize,
	// TODO i'm thinking of adding user constraints as well so that people can define their own 
	// constraints
	constraints:BoxContraints,
	children:Vec<Box<dyn Layout>>
}

impl HorizontalLayout {
	pub fn new() -> Self{
		Self::default()
	}

	pub fn add_child(&mut self,child:impl Layout + 'static){
		self.children.push(Box::new(child));
	}

	/// Calculate the sum of the width's of all nodes with fixed sizes and the max height
	fn fixed_size_sum(&self) -> Size{
		let mut sum = Size::default();

		for child in &self.children{
			match child.intrinsic_size().width {
				BoxSizing::Fixed(width) => {
					sum.width += width;
				},
				_ => {}
			}

			match child.intrinsic_size().height {
				BoxSizing::Fixed(height) => {
					// TODO not sure about this
					sum.height = sum.height.max(height);
				},
				_ => {}
			}
		}

		sum
	}

}


impl Layout for HorizontalLayout {
	fn size(&self) -> Size {
		self.size
	}

	fn children(&self) -> &[Box<dyn Layout>] {
		self.children.as_slice()
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
		// Sum up all the flex factors
		let flex_width_total:u8 = 
			self
			.children
			.iter()
			.filter_map(|child|{
				if let BoxSizing::Flex(factor) = child.intrinsic_size().width  {
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
				if let BoxSizing::Flex(factor) = child.intrinsic_size().height  {
					Some(factor)				
				}else {
					None
				}
			})
			.sum();
		
		for child in &mut self.children{
			let mut max_size = Size::default();

			match child.intrinsic_size().width {
				BoxSizing::Flex(factor) => {
					// Make sure the factor isn't bigger than available size
					let grow_factor = 
						factor as f32 / flex_width_total as f32;
					
					max_size.width = grow_factor * space.width;
					child.set_max_width(max_size.width);
					
					// TODO replace with custom err 
					assert_ne!(grow_factor,INFINITY);
					
				}
				_ => {}
			}

			match child.intrinsic_size().height {
				BoxSizing::Flex(factor) => {
					let grow_factor = 
						factor as f32 / flex_height_total as f32;

					max_size.height = grow_factor * space.height;
					child.set_max_height(max_size.height);
					
					assert_ne!(grow_factor,INFINITY);					
				},
				_ => {}
			}

			// Pass the max size to the children to solve their max constraints
			child.solve_max_contraints(max_size);
		}
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

		for child in &mut self.children{
			child.update_size();
		}
	}
}

#[cfg(test)]
mod test{
	use crate::LayoutSolver;
	use super::*;

	#[test]
	fn test_horizontal_layout(){
		let window = Size::new(800.0, 800.0);
		let mut root = HorizontalLayout::new();
		let mut child_1 = HorizontalLayout::new();
		let mut child_2 = HorizontalLayout::new();

		
		child_1.intrinsic_size.width = BoxSizing::Fixed(400.0);
		child_1.intrinsic_size.height = BoxSizing::Fixed(200.0);
		
		child_2.intrinsic_size.width = BoxSizing::Fixed(500.0);
		child_2.intrinsic_size.height = BoxSizing::Fixed(350.0);

		root.add_child(child_1);
		root.add_child(child_2);
		
		let mut solver = LayoutSolver::new(root);
		solver.solve(window);
		
		assert_eq!(
			solver.root.size(),
			Size::new(900.0, 350.0)
		);

		assert_eq!(
			solver.root.children()[0].size(),
			Size::new(400.0, 200.0)
		);

		assert_eq!(
			solver.root.children()[1].size(),
			Size::new(500.0, 350.0)
		);
	}
	
	#[test]
	fn test_flex_sizing(){
		let window = Size::new(800.0, 800.0);
		let mut root = HorizontalLayout::new();
		let mut child_1 = HorizontalLayout::new();
		let mut child_2 = HorizontalLayout::new();

		
		child_1.intrinsic_size.width = BoxSizing::Flex(1);
		child_1.intrinsic_size.height = BoxSizing::Flex(1);
		
		child_2.intrinsic_size.width = BoxSizing::Flex(1);
		child_2.intrinsic_size.height = BoxSizing::Flex(1);

		root.intrinsic_size.width = BoxSizing::Flex(1);
		root.intrinsic_size.height = BoxSizing::Flex(1);
		
		root.add_child(child_1);
		root.add_child(child_2);
		
		let mut solver = LayoutSolver::new(root);
		solver.solve(window);
		
		assert_eq!(solver.root.size(),window);
		assert_eq!(solver.root.children()[0].size(),window/2.0);
		assert_eq!(solver.root.children()[1].size(),window/2.0);
	}

	#[test]
	fn test_flex_inside_shrink(){
		// Child should have zero width
	}

	// TODO test flex grow inside flex shrink
	#[test]
	fn test_flex_factor(){
		let window = Size::new(800.0, 400.0);
		let mut node = HorizontalLayout::new();
		let mut child_node_1 = HorizontalLayout::new();
		let mut child_node_2 = HorizontalLayout::new();
		
		child_node_1.intrinsic_size.width = BoxSizing::Flex(1);
		child_node_1.intrinsic_size.height = BoxSizing::Flex(1);
		
		child_node_2.intrinsic_size.width = BoxSizing::Flex(3);
		child_node_2.intrinsic_size.height = BoxSizing::Flex(3);
		
		node.intrinsic_size.width = BoxSizing::Flex(1);		
		node.intrinsic_size.height = BoxSizing::Flex(1);		

		node.add_child(child_node_1);
		node.add_child(child_node_2);

		let mut solver = LayoutSolver::new(node);
		solver.solve(window);

	
		let flex_1_width = 1.0/4.0 * window.width;
		// The two children should both be half the size
		assert_eq!(
			solver.root.children()[0].size().width,
			flex_1_width
		);
		assert_eq!(solver.root.children()[0].size().height,400.0);
		assert_eq!(
			solver.root.children()[0].size().height,
			solver.root.children()[1].size().height,
		);
		assert!(
			solver.root.children()[1].size().width == 3.0 * solver.root.children()[0].size().width
		);
		assert!(
			solver.root.children()[1].size().height != 3.0 * solver.root.children()[0].size().height
		);
	}

	#[test]
	fn test_inner_grow_sizing(){
		// let window = Size::new(800.0, 400.0);
		// let mut root_node = HorizontalLayout::new();
		// let mut child_node = HorizontalLayout::new();
		// let mut grand_child_node = HorizontalLayout::new();
		
		// root_node.intrinsic_size.width = BoxSizing::Flex(1);
		// root_node.intrinsic_size.height = BoxSizing::Flex(1);
		
		// child_node.intrinsic_size.width = BoxSizing::Flex(1);
		// child_node.intrinsic_size.height = BoxSizing::Flex(1);
		
		// grand_child_node.intrinsic_size.width = BoxSizing::Flex(1);
		// grand_child_node.intrinsic_size.height = BoxSizing::Flex(1);
		
		// let mut first_node = child_node.clone();
		// first_node.add_child(grand_child_node.clone());
		// first_node.add_child(grand_child_node.clone());
		// first_node.add_child(grand_child_node.clone());

		// root_node.add_child(first_node);
		// root_node.add_child(child_node);

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
		// let window = Size::new(800.0, 400.0);
		// let mut root_node = LayoutNode::new();
		// let mut child_node = LayoutNode::new();

		// child_node.intrinsic_size.width = BoxSizing::Fixed(200.0);
		// child_node.intrinsic_size.height = BoxSizing::Fixed(50.0);

		// root_node.add_child(child_node.clone());
		// root_node.add_child(child_node);

		// let mut solver = LayoutSolver::new(root_node);

		// solver.solve(window);

		// assert_eq!(solver.root.size,Size::new(400.0, 100.0));
		// assert_eq!(solver.root.children[0].size,Size::new(200.0, 50.0));
	}

	#[test]
	fn test_inner_shrink_sizing(){
		// let window = Size::new(800.0, 800.0);

		// let mut root_node = LayoutNode::new();
		// let mut child_node = LayoutNode::new();
		// let mut grand_child_node = LayoutNode::new();

		// grand_child_node.intrinsic_size.width = BoxSizing::Fixed(50.0);
		// grand_child_node.intrinsic_size.height = BoxSizing::Fixed(200.0);
		
		// child_node.add_child(grand_child_node.clone());
		// child_node.add_child(grand_child_node);
		
		// root_node.add_child(child_node.clone());
		// root_node.add_child(child_node);

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

	#[test]
	fn test_positioning(){
		todo!()
	}
}