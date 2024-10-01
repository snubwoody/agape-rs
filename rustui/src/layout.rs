use crate::utils::Size;
use crate::widgets::{WidgetBody, WidgetTree};

/// The types of layout a [`Widget`] can have.
#[derive(Debug,Clone, Copy)]
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
	pub fn arrange_widgets(&self,widgets:&mut Vec<Box<WidgetBody>>) -> Size{
		match self {
			Self::Horizontal { spacing, padding } => self.arrange_horizontal(widgets,*padding,*spacing),
			Self::Vertical { spacing, padding } => self.arrange_vertical(widgets,*padding,*spacing),
			Self::Block { padding } => self.arrange_block(widgets,*padding),
		}
	}

	fn arrange_horizontal(&self,widgets:&mut Vec<Box<WidgetBody>>,padding:u32,spacing:u32) -> Size{
		// Set the initial position to the padding
		let mut current_pos = padding;
		
		let mut min_width:f32 = (padding * 2) as f32;
		let mut min_height:f32 = 0.0;

		for (_,widget) in widgets.iter_mut().enumerate(){
			// Set the current widget position
			widget.surface.position(current_pos as f32, padding as f32);
			
			// Add the spacing and the widget's width to the current
			// position and the min width
			current_pos += spacing;
			current_pos += widget.surface.get_size().width as u32;

			min_width += spacing as f32;
			min_width += widget.surface.get_size().width;

			// Set the minimum height to the height of the largest widget
			min_height = min_height.max(widget.surface.get_size().height);

			// Arrange the widget's children recursively
			widget.layout.arrange_widgets(&mut widget.children);
		}

		Size::new(min_width, min_height + (padding * 2) as f32)
	}

	fn arrange_vertical(&self,widgets:&mut Vec<Box<WidgetBody>>,padding:u32,spacing:u32) -> Size{
		// Set the initial position to the padding
		let mut current_pos = padding;

		for (_,widget) in widgets.iter_mut().enumerate(){
			widget.surface.position(padding as f32, current_pos as f32);
			
			// Add the spacing and the widget's width to the current
			// position
			current_pos += spacing;
			current_pos += widget.surface.get_size().height as u32;

			// Arrange the widget's children recursively
			widget.layout.arrange_widgets(&mut widget.children);
		}
		
		Default::default()
	}

	fn arrange_block(&self,widgets:&mut Vec<Box<WidgetBody>>,padding:u32) -> Size{
		// Block elements should only have one child
		// so no need to iterate
		if widgets.is_empty() {
			return Default::default();
		}
		widgets[0].surface.position(padding as f32, padding as f32);

		Default::default()
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

