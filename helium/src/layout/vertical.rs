use std::f32::INFINITY;

use helium_core::{position::Position, size::Size};
use crate::widgets::WidgetBody;
use super::{AxisAlignment, BoxContraints, IntrinsicSize, Layout, LayoutType, WidgetSize};

#[derive(Debug,Clone, Copy,Default)]
pub struct VerticalLayout{
	spacing:u32,
	padding:u32,
	postision:Position,
	size:Size,
	main_axis_alignment:AxisAlignment,
	cross_axis_alignment:AxisAlignment,
	constraints:BoxContraints,
	intrinsic_size:IntrinsicSize,
}

impl VerticalLayout {
	pub fn new(spacing:u32,padding:u32) -> Self{
		Self { 
			spacing,
			padding, 
			..Default::default()
		}
	}

	pub fn spacing(&mut self,spacing:u32){
		self.spacing = spacing;
	}

	pub fn padding(&mut self,padding:u32){
		self.padding = padding;
	}

	pub fn intrinsic_size(&mut self,intrinsize_size:IntrinsicSize){
		self.intrinsic_size = intrinsize_size;
	}

	fn calculate_min_constraints(&mut self){
		
	}
}

impl Layout for VerticalLayout {
	/// Computes the layout based on the `widget`'s layout attributes
	/// and returns the minimum size required to fit the widget's children.
	/// This function runs recursively for each `widget`.
	fn compute_layout(
		&mut self,
		widgets:&mut Vec<Box<WidgetBody>>,
		available_space:Size,
		parent_pos:Position
	) -> Size{
		self.calculate_min_constraints();

		match self.intrinsic_size.width {
			WidgetSize::Fill => {
				self.size.width = self.constraints.max_width
			},
			WidgetSize::Fit => {
				self.size.width = self.constraints.min_width
			},
			WidgetSize::Fixed(_) => {
				// TODO check this
				self.size.width = self.constraints.min_width
			}
		}
		
		match self.intrinsic_size.height {
			WidgetSize::Fill => {
				self.size.height = self.constraints.max_height
			},
			WidgetSize::Fit => {
				self.size.height = self.constraints.min_height
			},
			WidgetSize::Fixed(_) => {
				// TODO check this
				self.size.height = self.constraints.min_height
			}
		}	
		
		self.align(widgets, &parent_pos);

		// FIXME
		Size::default()
	}

	/// Position the `Widgets` according to the [`AxisAlignment`]
	fn align(&self,widgets:&mut Vec<Box<WidgetBody>>,parent_pos:&Position){
		let mut pos = parent_pos.clone();
		// Add the padding
		pos += self.padding as f32;

		for widget in widgets{
			// Set the current widget position
			widget.surface.position(pos.x,pos.y);

			// Add the spacing and the widget's width to the current
			// position and the min width
			pos.y += self.spacing as f32;
			pos.y += widget.surface.get_size().height;
			self.align(&mut widget.children, &widget.surface.get_position());
		}
	}
}		
