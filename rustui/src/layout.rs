use crate::utils::{Position, Size};
use crate::widgets::WidgetBody;

/// The types of layout a [`Widget`] can have.
#[derive(Debug,Clone,Copy)]
pub enum Layout{
	Horizontal{
		spacing:u32,
		padding:u32,
	},
	Vertical{
		spacing:u32,
		padding:u32,
	},
	Block{
		padding:u32,
	},
}

impl Layout {
	/// Arrange and size the widgets.
	pub fn arrange_widgets(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		max_size:Size,
		parent_pos:Position
	) -> Size{
		match self {
			Self::Horizontal { spacing, padding } => 
				self.arrange_horizontal(widgets,*padding,*spacing,&max_size,&parent_pos),
			Self::Vertical { spacing, padding } => 
				self.arrange_vertical(widgets,*padding,*spacing,&max_size,&parent_pos),
			Self::Block { padding } => 
				self.arrange_block(widgets,*padding,&max_size,&parent_pos),
		}
	}

	fn arrange_horizontal(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		padding:u32,
		spacing:u32,
		max_size:&Size,
		parent_pos:&Position
	) -> Size{
		// Set the initial position to the padding plus 
		// the parent position
		let mut current_pos = padding as f32 + parent_pos.x;
		
		let mut min_width:f32 = (padding * 2) as f32;
		let mut min_height:f32 = 0.0;

		for (_,widget) in widgets.iter_mut().enumerate(){
			// Set the current widget position
			widget.surface.position(current_pos as f32, parent_pos.y + padding as f32);

			// Arrange the widget's children recursively and return the min size
			let size = widget.layout.arrange_widgets(
				&mut widget.children,
				*max_size,
				Position::new(
					widget.surface.get_position().0,
					widget.surface.get_position().1
				)
			);

			// Set the widget's size
			match widget.intrinsic_size.width {
				WidgetSize::Fill => widget.surface.width(max_size.width),
				WidgetSize::Fit => widget.surface.width(size.width),
				WidgetSize::Fixed(size) => widget.surface.width(size),
			}

			match widget.intrinsic_size.height {
				WidgetSize::Fill => widget.surface.height(max_size.height),
				WidgetSize::Fit => widget.surface.height(size.height),
				WidgetSize::Fixed(size) => widget.surface.height(size),
			}

			// Add the spacing and the widget's width to the current
			// position and the min width
			current_pos += spacing as f32;
			current_pos += widget.surface.get_size().width;

			min_width += spacing as f32;
			min_width += widget.surface.get_size().width;

			// Set the minimum height to the height of the largest widget
			min_height = min_height.max(widget.surface.get_size().height);
		}

		Size::new(min_width, min_height + (padding * 2) as f32)
	}

	fn arrange_vertical(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		padding:u32,
		spacing:u32,
		max_size:&Size,
		parent_pos:&Position
	) -> Size{
		// Set the initial position to the padding
		let mut current_pos = padding;

		for (_,widget) in widgets.iter_mut().enumerate(){
			widget.surface.position(padding as f32, current_pos as f32);
			
			// Add the spacing and the widget's width to the current
			// position
			current_pos += spacing;
			current_pos += widget.surface.get_size().height as u32;

			// Arrange the widget's children recursively
			let size = widget.layout.arrange_widgets(
				&mut widget.children,
				*max_size,
				Position::new(
					widget.surface.get_position().0,
					widget.surface.get_position().1
				)

			);
		}
		
		Size::new(0.0, 500.0)
	}

	fn arrange_block(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		padding:u32,
		max_size:&Size,
		parent_pos:&Position
	) -> Size{
		// Return if element has no children
		if widgets.is_empty() {
			Default::default()
		}

		let mut min_width = padding as f32 * 2.0;
		let mut min_height = padding as f32 * 2.0;

		for (_,widget) in widgets.iter_mut().enumerate(){
			widget.surface.position(
				parent_pos.x + padding as f32, 
				parent_pos.y + padding as f32
			);

			// If the widget has no children return
			// it's size 
			let size = if widget.children.is_empty() {
				widget.surface.get_size()
			} else{
				widget.layout.arrange_widgets(
					&mut widget.children, 
					*max_size,
					Position::new(
						widget.surface.get_position().0,
						widget.surface.get_position().1
					)
				)
			};

			//dbg!(&size,&widget);

			min_width += size.width;
			min_height += size.height;
			
			// Set the widget's size
			match widget.intrinsic_size.width {
				WidgetSize::Fill => widget.surface.width(max_size.width),
				WidgetSize::Fit => widget.surface.width(size.width),
				WidgetSize::Fixed(size) => widget.surface.width(size),
			}

			match widget.intrinsic_size.height {
				WidgetSize::Fill => widget.surface.height(max_size.height),
				WidgetSize::Fit => widget.surface.height(size.height),
				WidgetSize::Fixed(size) => widget.surface.height(size),
			}
		}

		
		Size::new(min_width, min_height)
	}
}

#[derive(Debug,Clone, Copy,Default)]
pub enum WidgetSize{
	Fixed(f32),
	Fill,
	#[default]
	Fit,
}


#[derive(Debug,Clone,Copy,Default)]
pub struct IntrinsicSize {
	pub width:WidgetSize,
	pub height:WidgetSize
}

/// The [`Widget`] constraints that are used when calculating
/// it's size.
#[derive(Debug,Clone,Copy,PartialEq, PartialOrd,Default)]
pub struct Constraint{
	pub max_width:f32,
	pub min_width:f32,
	pub max_height:f32,
	pub min_height:f32,
}

impl Constraint {
	pub fn new(max_width:f32,min_width:f32,max_height:f32,min_height:f32) -> Self{
		Self { max_width, min_width, max_height, min_height }
	}
}

