use helium_core::{position::Position, size::Size};
use crate::widgets::WidgetBody;
use super::{LayoutType,AxisAlignment,LayoutHandler,WidgetSize};

pub struct BlockLayout{
	spacing:u32, // TODO remove this
	padding:u32,
	layout:LayoutType,
	main_axis_alignment:AxisAlignment,
	cross_axis_alignment:AxisAlignment
}

impl LayoutHandler for BlockLayout {
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
			// Subtract the spacing for every element except the last
			if i != widgets.len() - 1{
				size.width -= self.spacing as f32; // TEMP
				size.height -= self.spacing as f32; // TEMP
			}

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

		for widget in widgets{
			// Set the current widget position
			match self.layout {
				LayoutType::Vertical => {
					widget.surface.position(parent_pos.y + self.padding as f32,current_pos);
					// Add the spacing and the widget's width to the current
					// position and the min width
					current_pos += self.spacing as f32;
					current_pos += widget.surface.get_size().height;
				},
				LayoutType::Horizontal => {
					widget.surface.position(current_pos as f32, parent_pos.y + self.padding as f32);
					// Add the spacing and the widget's width to the current
					// position and the min width
					current_pos += self.spacing as f32;
					current_pos += widget.surface.get_size().width;
				}
				LayoutType::Block => {}
			}
			self.align(&mut widget.children, &widget.surface.get_position());
		}
	}
}		
