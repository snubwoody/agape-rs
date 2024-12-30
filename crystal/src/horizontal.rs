use std::f32::INFINITY;
use helium_core::{position::Position, size::Size};
use crate::{AxisAlignment, BoxContraints, BoxSizing, IntrinsicSize, Layout, LayoutIter};

/// A [`HorizontalLayout`] sizes and position it's children horizontally, of course, the `Flex` 
/// attribute means a layout node will fill it's widget, however the flex factor only works in 
/// the x-axis, in the y-axis all nodes will fill the parent and will be the same height.
#[derive(Default,Debug)]
pub struct HorizontalLayout{
	pub id:String,
	pub size:Size,
	pub position:Position,
	pub spacing:u32,
	pub padding:u32,
	// TODO i'm thinking of adding user constraints as well so that people can define their own 
	// constraints
	pub constraints:BoxContraints,
	pub intrinsic_size:IntrinsicSize,
	/// The main axis is the axis which the content flows in, for the [`HorizontalLayout`]
	/// main axis is the `x-axis`
	pub main_axis_alignment:AxisAlignment,
	/// The cross axis is the `y-axis`
	pub cross_axis_alignment:AxisAlignment,
	pub children:Vec<Box<dyn Layout>>,
	pub errors:Vec<crate::LayoutError>
}

impl HorizontalLayout {
	pub fn new() -> Self{
		Self::default()
	}

	pub fn add_child(&mut self,child:impl Layout + 'static){
		self.children.push(Box::new(child));
	}
	
	pub fn add_children<I>(&mut self,children:I)
	where 
		I:IntoIterator<Item:Layout + 'static>
	{
		for child in children{
			self.children.push(Box::new(child));
		}
	}

	/// Calculate the sum of the width's of all nodes with fixed sizes and the max height
	fn fixed_size_sum(&self) -> Size{
		let mut sum = Size::default();

		// TODO should probably rename this function
		for (i,child) in self.children.iter().enumerate(){
			match child.intrinsic_size().width {
				BoxSizing::Fixed(width) => {
					sum.width += width;
				},
				BoxSizing::Shrink => {
					sum.width += child.constraints().min_width;
				}
				_ => {}
			}

			match child.intrinsic_size().height {
				BoxSizing::Fixed(height) => {
					// TODO not sure about this
					sum.height = sum.height.max(height);
				},
				_ => {}
			}

			// Add the spacing between layouts
			if i != self.children.len() - 1 {
				sum.width += self.spacing as f32;
			} 
		}

		sum
	}

	/// Align the children on the main axis in the center
	fn align_cross_axis_center(&mut self){
		let mut current_pos = self.position;
		current_pos += self.padding as f32;
		
		for child in &mut self.children{
			// TODO handle overflow
			let y_pos = (self.size.height - child.size().height) / 2.0 + self.position.y;
			child.set_y(y_pos);
			
		}
	}

	/// Align the children on the main axis in the center
	fn align_main_axis_center(&mut self){
		// TODO handle overflow
		let mut width_sum = 
			self.children.iter().map(|child|child.size().width).sum::<f32>();
		width_sum += (self.spacing * (self.children.len() as u32 - 1)) as f32;
		let mut center_start = self.position.x + (self.size.width - width_sum)/2.0;

		for child in &mut self.children{
			child.set_x(center_start);
			center_start += child.size().width + self.spacing as f32;
		}
	}
}


impl Layout for HorizontalLayout {
	fn id(&self) -> &str {
		&self.id
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
	
	fn size(&self) -> Size {
		self.size
	}

	fn position(&self) -> Position {
		self.position
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

	fn collect_errors(&self) -> Vec<crate::LayoutError> {
		self.errors
		.iter()
		.cloned()
		.chain(
			self
			.children
			.iter()
			.flat_map(|child|child.collect_errors())
			.collect::<Vec<_>>()
		).collect::<Vec<_>>()
	}

	fn sort_children(&mut self) {
		// FIXME this is messing with the order of the children, so probably just return?
		// self.children.sort_by(|a,b|
		// 	a.intrinsic_size().width.partial_cmp(&b.intrinsic_size().width).unwrap()
		// );
	}

	fn iter(&self) -> crate::LayoutIter {
		LayoutIter{
			stack:vec![Box::new(self)]
		}
	}

	fn solve_min_constraints(&mut self) -> (f32,f32){
		// The sum of the size of all the children with fixed sizes
		let mut fixed_sum = self.fixed_size_sum();
		fixed_sum += self.padding as f32 * 2.0;

		let mut child_constraint_sum = Size::default();

		for child in &mut self.children{
			let (min_width,min_height) = child.solve_min_constraints();
			child_constraint_sum.width += min_width;
			child_constraint_sum.width += self.spacing as f32; // Not sure about this
			child_constraint_sum.height = child_constraint_sum.height.max(min_height);
		}
		child_constraint_sum += self.padding as f32 * 2.0;

		// TODO i think im supposed to calculate the min constraints of the children as well
		match self.intrinsic_size.width {
			BoxSizing::Fixed(width) => {
				self.constraints.min_width = width;	
			},
			BoxSizing::Flex(_) => {
				// TODO maybe set the min constraints to either 0 or the size of the children
				self.constraints.min_width = child_constraint_sum.width;	
			},
			BoxSizing::Shrink => {
				self.constraints.min_width = child_constraint_sum.width;	
			},
		}
		
		match self.intrinsic_size.height {
			BoxSizing::Fixed(height) => {
				self.constraints.min_height = height;	
			},
			BoxSizing::Flex(_) => {
				self.constraints.min_height = child_constraint_sum.height;	
			},
			BoxSizing::Shrink => {
				self.constraints.min_height = child_constraint_sum.height;	
			},
		}
		

		(self.constraints.min_width,self.constraints.min_height)
	}

	// TODO add custom errors for negative and infinite spacing
	fn solve_max_contraints(&mut self,space:Size) {
		// Sum up all the flex factors
		let flex_total:u8 = 
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
			
		let mut available_space = Size{
			width:self.constraints.max_width,
			height:self.constraints.max_height
		};
		available_space -= self.padding as f32 * 2.0;
		available_space.width -= self.fixed_size_sum().width;

		// TODO subtract the spacing
		// TODO currently the min constraints are bigger then max constraints
		// for shrink nodes, which doesn't make any sense.
		for child in &mut self.children{
			match child.intrinsic_size().width {
				BoxSizing::Flex(factor) => {
					// Make sure the factor isn't bigger than available size
					let grow_factor = 
						factor as f32 / flex_total as f32;
					
					child.set_max_width(grow_factor * available_space.width);
					
					// TODO replace with custom err 
					assert_ne!(grow_factor,INFINITY);
					
				}
				BoxSizing::Fixed(width) => {
					child.set_max_width(width);
				}
				BoxSizing::Shrink => {
					// Not sure about this
					child.set_max_width(child.constraints().min_width);
				}
			}

			match child.intrinsic_size().height {
				BoxSizing::Flex(_) => {
					// TODO Maybe set to min constraints?
					let available_height = 
						self.constraints.max_height - self.padding as f32 * 2.0;
					child.set_max_height(available_height);
				},
				BoxSizing::Fixed(height) => {
					child.set_max_height(height);
				}
				BoxSizing::Shrink => {
					child.set_max_height(child.constraints().min_height);
				}
			}


		
			// Pass the max size to the children to solve their max constraints
			let space = Size{
				width:child.constraints().max_width,
				height:child.constraints().max_height
			};
			// TODO not even using the space anymore
			child.solve_max_contraints(space);
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

	fn position_children(&mut self){
		let mut current_pos = self.position;
		current_pos += self.padding as f32;
		
		match self.main_axis_alignment {
			AxisAlignment::Center => self.align_main_axis_center(),
			AxisAlignment::Start | AxisAlignment::End => {
				for child in &mut self.children{
					child.set_x(current_pos.x);
					current_pos.x += child.size().width + self.spacing as f32;
				}
			}
		}

		match self.cross_axis_alignment {
			AxisAlignment::Center => self.align_cross_axis_center(),
			AxisAlignment::Start | AxisAlignment::End => {
				for child in &mut self.children{
					child.set_y(current_pos.y);
				}
			} 
		}
		
		for child in &mut self.children{
			if child.position().x > self.position.x + self.size.width{
				log::warn!("Child out of bounds")
			}
			child.position_children();
		}

	}
}
