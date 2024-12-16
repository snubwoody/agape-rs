use helium_core::{position::Position, size::Size};
use crate::widgets::WidgetBody;
use super::{AxisAlignment,Layout,WidgetSize};

#[derive(Debug,Clone,Copy)]
pub struct BlockLayout{
	padding:u32,
	main_axis_alignment:AxisAlignment,
	cross_axis_alignment:AxisAlignment
}

impl BlockLayout {
	pub fn new(padding:u32) -> Self{
		Self { 
			padding, 
			main_axis_alignment: AxisAlignment::Start, 
			cross_axis_alignment: AxisAlignment::Start 
		}
	}

	pub fn padding(&mut self,padding:u32){
		self.padding = padding;
	}
}

impl Layout for BlockLayout {
	fn compute_layout(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		max_size:Size,
		parent_pos:Position
	) -> Size {
		let mut min_width = self.padding as f32 * 2.0;
		let mut min_height = self.padding as f32 * 2.0;
		let child_max_size = self.max_size(widgets, max_size);

		if widgets.is_empty(){
			return Size::default()
		}

		for (i,widget) in widgets.iter_mut().enumerate(){
			widget.surface.position(
				parent_pos.x + self.padding as f32, 
				parent_pos.y + self.padding as f32
			);

			let size = widget.layout.compute_layout(
				&mut widget.children, 
				max_size,
				widget.surface.get_position()
			);
			
			// Set the widget's size
			match widget.intrinsic_size.width {
				WidgetSize::Fill => widget.surface.width(child_max_size.width),
				WidgetSize::Fit => widget.surface.width(size.width),
				WidgetSize::Fixed(size) => widget.surface.width(size),
			}
			
			match widget.intrinsic_size.height {
				WidgetSize::Fill => widget.surface.height(child_max_size.height),
				WidgetSize::Fit => widget.surface.height(size.height),
				WidgetSize::Fixed(size) => widget.surface.height(size),
			}
			
			min_width += widget.surface.get_size().width;
			min_height += widget.surface.get_size().height;
		};
		
		Size::new(min_width, min_height)
	}

	fn max_size(&self,widgets:&[Box<WidgetBody>],max_size:Size) -> Size {
		// The maximum size for the widget children to be
		let mut size = max_size;

		// The number of widgets that have that their size set to fill
		let mut width_fill_count = 0;
		let mut height_fill_count = 0;

		size.width -= (self.padding * 2) as f32;
		size.height -= (self.padding * 2) as f32;
		
		for (i,widget) in widgets.iter().enumerate(){
			// TODO maybe move this to the enum?
			match widget.intrinsic_size.width {
				WidgetSize::Fixed(width) => {
					size.width -= width
				}
				WidgetSize::Fit => {
					size.width += widget.surface.get_size().width;
				},
				WidgetSize::Fill => {
					width_fill_count += 1;
				}
			}

			match widget.intrinsic_size.height {
				WidgetSize::Fixed(height) => {
					size.height -= height
				}
				WidgetSize::Fit => {
					size.height += widget.surface.get_size().height;
				},
				WidgetSize::Fill => {
					height_fill_count += 1;
				}
			}
		};

		// Distribute the size evenly among the children 
		size.width /= width_fill_count as f32;
		size.height /= height_fill_count as f32;

		size
	}

	/// Position the `Widgets` according to the [`AxisAlignment`]
	fn align(&self,widgets:&mut Vec<Box<WidgetBody>>,parent_pos:&Position){
		let mut current_pos = self.padding as f32 + parent_pos.x;
	}
}		
