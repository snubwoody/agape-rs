use helium_core::{position::Position, size::Size};
use crate::widgets::WidgetBody;
use super::{LayoutType,AxisAlignment,Layout,WidgetSize};

#[derive(Debug,Clone, Copy)]
pub struct VerticalLayout{
	spacing:u32,
	padding:u32,
	main_axis_alignment:AxisAlignment,
	cross_axis_alignment:AxisAlignment
}

impl VerticalLayout {
	pub fn new(spacing:u32,padding:u32) -> Self{
		Self { 
			spacing,
			padding, 
			main_axis_alignment: AxisAlignment::Start, 
			cross_axis_alignment: AxisAlignment::Start 
		}
	}

	pub fn spacing(&mut self,spacing:u32){
		self.spacing = spacing;
	}

	pub fn padding(&mut self,padding:u32){
		self.padding = padding;
	}
}

impl Layout for VerticalLayout {
	/// Computes the layout based on the `widget`'s layout attributes
	/// and returns the minimum size required to fit the widget's children.
	/// This function runs recursively for each `widget`.
	fn compute_layout(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		available_space:Size,
		parent_pos:Position
	) -> Size{
		// Set the initial position to the padding plus 
		// the parent position
		let mut min_width:f32 = 0.0;
		let mut min_height:f32 = 0.0;
		let length = widgets.len();

		if widgets.is_empty(){
			return Size::default()
		}

		let child_max_size = self.available_space(widgets, available_space);

		// TODO the same max size gets passes everytime that can't be right.
		for (i,widget) in widgets.iter_mut().enumerate(){
			// Arrange the widget's children recursively and return the min size
			let size = widget.layout.compute_layout(
				&mut widget.children,
				available_space,
				widget.surface.get_position()
			);

			// TODO maybe create a set_size fn in the trait
			// Set the widget's size
			match widget.intrinsic_size.width {
				WidgetSize::Fill => widget.surface.width(child_max_size.width),
				WidgetSize::Fit => widget.surface.width(size.width),
				WidgetSize::Fixed(width) => widget.surface.width(width),
			}

			match widget.intrinsic_size.height {
				WidgetSize::Fill => widget.surface.height(child_max_size.height),
				WidgetSize::Fit => widget.surface.height(size.height),
				WidgetSize::Fixed(height) => widget.surface.height(height),
			}

			if i != length - 1{
				min_height += self.spacing as f32;
			}

			min_height += widget.surface.get_size().height;
			// Set the minimum width to the width of the largest widget
			min_width = min_width.max(widget.surface.get_size().width);
		};

		self.align(widgets, &parent_pos);

		min_width += (self.padding * 2) as f32;
		min_height += (self.padding * 2) as f32;
		
		Size::new(min_width,min_height)
	}

	fn available_space(&self,widgets:&[Box<WidgetBody>],available_space:Size) -> Size {
		// The maximum size for the widget children to be
		let mut size = available_space;

		// The number of widgets that have that their size set to fill
		let mut width_fill_count = 0;
		let mut height_fill_count = 0;

		size.width -= (self.padding * 2) as f32;
		size.height -= (self.padding * 2) as f32;
		
		for (i,widget) in widgets.iter().enumerate(){
			// Subtract the spacing for every element except the last
			if i != widgets.len() - 1{
				size.width -= self.spacing as f32; // TEMP
				size.height -= self.spacing as f32; // TEMP
			}

			// TODO maybe move this to the enum?
			match widget.intrinsic_size.width {
				WidgetSize::Fill => {
					width_fill_count -= 1;
				},
				_ => {
					size.width += widget.surface.get_size().width;
				}
			}

			match widget.intrinsic_size.height {
				WidgetSize::Fill => {
					height_fill_count += 1;
				},
				_=> {
					size.height -= widget.surface.get_size().height;
				},
				
			}
		};

		// Distribute the size evenly among the children 
		size.width /= width_fill_count as f32;
		size.height /= height_fill_count as f32;

		size
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
