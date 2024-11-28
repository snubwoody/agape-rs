use helium_core::{position::Position, size::Size};
use crate::widgets::WidgetBody;

/// How a [`Widget`] should align it's children
#[derive(Debug,Clone, Copy,PartialEq,Eq,Default)]
pub enum AxisAlignment{
	#[default]
	Start,
	Center,
	End,
	/// The parent [`Widget`] will attempt to have as much spacing 
	/// between the widgets as possible.
	SpaceBetween,
	/// The parent [`Widget`] will attempt to have as little spacing 
	/// between the widgets as possible.
	SpaceAround,
	/// The parent [`Widget`] will attempt to have equal spacing 
	/// both around and between the widgets.
	SpaceEvenly
}

#[derive(Debug,Clone, Copy,PartialEq, Eq,Default)]
pub enum LayoutType {
	#[default]
	Block,
	Horizontal,
	Vertical,
}

#[derive(Debug,Clone, Copy,Default)]
pub struct LayoutM{
	spacing:u32,
	padding:u32,
	layout:LayoutType,
	main_axis_alignment:AxisAlignment,
	cross_axis_alignment:AxisAlignment
}

impl LayoutM {
	pub fn new() -> Self{
		Self::default()
	}

	pub fn spacing(&mut self,spacing:u32){
		self.spacing = spacing;
	}

	pub fn padding(&mut self,padding:u32){
		self.padding = padding;
	}

	pub fn layout(&mut self,layout:LayoutType){
		self.layout = layout;
	}

	pub fn main_axis_alignment(&mut self,main_axis_alignment:AxisAlignment){
		self.main_axis_alignment = main_axis_alignment;
	}

	pub fn cross_axis_alignment(&mut self,cross_axis_alignment:AxisAlignment){
		self.cross_axis_alignment = cross_axis_alignment;
	}

	/// Arrange and size the widgets.
	pub fn arrange_widgets(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		max_size:Size,
		parent_pos:Position
	) -> Size{
		match self.layout {
			LayoutType::Horizontal => 
				self.arrange_horizontal(widgets,&max_size,&parent_pos),
			LayoutType::Vertical => 
				self.arrange_vertical(widgets,&max_size,&parent_pos),
			LayoutType::Block => 
				self.arrange_block(widgets,&max_size,&parent_pos),
		}
	}

	fn arrange_horizontal(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		max_size:&Size,
		parent_pos:&Position
	) -> Size{
		// Set the initial position to the padding plus 
		// the parent position
		let mut current_pos = self.padding as f32 + parent_pos.x;
		
		let mut min_width:f32 = (self.padding * 2) as f32;
		let mut min_height:f32 = 0.0;

		widgets.iter_mut().for_each(|widget|{
			// Set the current widget position
			widget.surface.position(current_pos as f32, parent_pos.y + self.padding as f32);

			// Arrange the widget's children recursively and return the min size
			let size = widget.layout.arrange_widgets(
				&mut widget.children,
				*max_size,
				Position::new(
					widget.surface.get_position().x,
					widget.surface.get_position().y
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
			current_pos += self.spacing as f32;
			current_pos += widget.surface.get_size().width;

			min_width += self.spacing as f32;
			min_width += widget.surface.get_size().width;

			// Set the minimum height to the height of the largest widget
			min_height = min_height.max(widget.surface.get_size().height);
		});

		Size::new(min_width, min_height + (self.padding * 2) as f32)
	}

	fn arrange_vertical(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		max_size:&Size,
		parent_pos:&Position
	) -> Size{
		// Set the initial position to the padding plus 
		// the parent position
		let mut current_pos = self.padding as f32 + parent_pos.y;
		
		let mut min_width:f32 = 0.0;
		let mut min_height:f32 = (self.padding * 2) as f32;

		widgets.iter_mut().for_each(|widget|{
			// Set the current widget position
			widget.surface.position(parent_pos.x + self.padding as f32,current_pos as f32);

			// Arrange the widget's children recursively and return the min size
			let size = widget.layout.arrange_widgets(
				&mut widget.children,
				*max_size,
				Position::new(
					widget.surface.get_position().x,
					widget.surface.get_position().y
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
			current_pos += self.spacing as f32;
			current_pos += widget.surface.get_size().height;

			min_height += self.spacing as f32;
			min_height += widget.surface.get_size().height;

			// Set the minimum height to the height of the largest widget
			min_width = min_width.max(widget.surface.get_size().width);
		});

		Size::new(min_width + (self.padding * 2) as f32,min_height)
	}

	fn arrange_block(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		max_size:&Size,
		parent_pos:&Position
	) -> Size{
		// Return if element has no children
		if widgets.is_empty() {
			Default::default()
		}

		let mut min_width = self.padding as f32 * 2.0;
		let mut min_height = self.padding as f32 * 2.0;

		widgets.iter_mut().for_each(|widget|{
			widget.surface.position(
				parent_pos.x + self.padding as f32, 
				parent_pos.y + self.padding as f32
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
						widget.surface.get_position().x,
						widget.surface.get_position().y
					)
				)
			};


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
		});

		
		Size::new(min_width, min_height)
	}
}

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

		widgets.iter_mut().for_each(|widget|{
			// Set the current widget position
			widget.surface.position(current_pos as f32, parent_pos.y + padding as f32);

			// Arrange the widget's children recursively and return the min size
			let size = widget.layout.arrange_widgets(
				&mut widget.children,
				*max_size,
				Position::new(
					widget.surface.get_position().x,
					widget.surface.get_position().y
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
		});

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
		// Set the initial position to the padding plus 
		// the parent position
		let mut current_pos = padding as f32 + parent_pos.y;
		
		let mut min_width:f32 = 0.0;
		let mut min_height:f32 = (padding * 2) as f32;

		widgets.iter_mut().for_each(|widget|{
			// Set the current widget position
			widget.surface.position(parent_pos.x + padding as f32,current_pos as f32);

			// Arrange the widget's children recursively and return the min size
			let size = widget.layout.arrange_widgets(
				&mut widget.children,
				*max_size,
				Position::new(
					widget.surface.get_position().x,
					widget.surface.get_position().y
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
			current_pos += widget.surface.get_size().height;

			min_height += spacing as f32;
			min_height += widget.surface.get_size().height;

			// Set the minimum height to the height of the largest widget
			min_width = min_width.max(widget.surface.get_size().width);
		});

		Size::new(min_width + (padding * 2) as f32,min_height)
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

		widgets.iter_mut().for_each(|widget|{
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
						widget.surface.get_position().x,
						widget.surface.get_position().y
					)
				)
			};


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
		});

		
		Size::new(min_width, min_height)
	}
}

#[derive(Debug,Clone, Copy,Default,PartialEq)]
pub enum WidgetSize{
	Fixed(f32),
	Fill,
	#[default]
	Fit,
}


/// This is the size that a [`Widget`] will try to be,  
/// the actual final size is dependant on the space
/// available.
#[derive(Debug,Clone,Copy,Default)]
pub struct IntrinsicSize {
	pub width:WidgetSize,
	pub height:WidgetSize
}


