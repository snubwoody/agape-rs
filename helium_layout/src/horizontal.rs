use std::f32::INFINITY;
use helium_core::{position::Position, size::Size};
use crate::{BoxContraints, BoxSizing, IntrinsicSize, Layout};

#[derive(Default)]
pub struct HorizontalLayout{
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

}


impl Layout for HorizontalLayout {
	fn constraints(&self) -> BoxContraints {
		self.constraints
	}

	fn intrinsic_size(&self) -> IntrinsicSize {
		self.intrinsic_size
	}

	fn solve_min_constraints(&mut self) -> (f32,f32){
		// The sum of the size of all the children with fixed sizes
		let fixed_sum = self.fixed_size_sum();

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
	
	#[test]
	fn test_flex_sizing(){
		todo!()
	}

	#[test]
	fn test_positioning(){
		todo!()
	}
}