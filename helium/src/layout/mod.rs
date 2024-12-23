mod horizontal;
mod vertical;
mod block;
use std::fmt::Debug;

use helium_core::{position::Position, size::Size};
use crate::widgets::WidgetBody;
pub use horizontal::HorizontalLayout;
pub use vertical::VerticalLayout;
pub use block::BlockLayout;


// TODO maybe pass return the size instead of the mut widgets

/// Handles the layout of `widgets`. It works by calculating the max size
/// which is the maximum size that widget's are allowed to be and the min size
/// which is the mininum space required to fit a widgets children. If a widget
/// is set to `fill` it will use the max size and if it is set to `fit` then it will
/// use the min size.
pub trait Layout:Debug {
	/// Computes the layout of the [`Widget`]'s i.e. the size
	/// and positioning.
	fn compute_layout(
		&mut self,
		widgets:&mut Vec<Box<WidgetBody>>,
		available_space:Size,
		parent_pos:Position
	) -> Size;

	fn align(&self,widgets:&mut Vec<Box<WidgetBody>>,parent_pos:&Position);
	fn available_space(&self,widgets:&[Box<WidgetBody>],available_space:Size) -> Size;
}


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

#[derive(Debug,Clone, Copy,Default,PartialEq)]
pub enum WidgetSize{
	Fixed(f32),
	/// Tries to be as big as possible
	Fill,
	#[default]
	/// Tries to be as small as possible
	Fit, // TODO maybe change these to grow and shrink
}

#[derive(Debug,Clone, Copy,Default)]
pub struct BoxContraints{
	pub max_width:f32,
	pub max_height:f32,
	pub min_height:f32,
	pub min_width:f32
}

impl BoxContraints {
	pub fn new() -> Self{
		Self::default()
	}
}

/// This is the size that a [`Widget`] will try to be,  
/// the actual final size is dependant on the space
/// available.
#[derive(Debug,Clone,Copy,Default)]
pub struct IntrinsicSize {
	pub width:WidgetSize,
	pub height:WidgetSize
}

impl IntrinsicSize {
	pub fn new() -> Self{
		Self { width: WidgetSize::Fit, height: WidgetSize::Fit }
	}

	pub fn fixed(mut self,width:u32,height:u32) -> Self{
		self.width = WidgetSize::Fixed(width as f32);
		self.height = WidgetSize::Fixed(height as f32);
		self
	}

	pub fn fixed_width(mut self,width:u32) -> Self{
		self.width = WidgetSize::Fixed(width as f32);
		self
	}

	pub fn fixed_height(mut self,height:u32) -> Self{
		self.height = WidgetSize::Fixed(height as f32);
		self
	}
	
	pub fn fit(mut self) -> Self {
		self.width = WidgetSize::Fit;
		self.height = WidgetSize::Fit;
		self
	}

	pub fn fit_width(mut self) -> Self {
		self.width = WidgetSize::Fit;
		self
	}

	pub fn fit_height(mut self) -> Self {
		self.height = WidgetSize::Fit;
		self
	}

	pub fn fill(mut self) -> Self{
		self.width = WidgetSize::Fill;
		self.height = WidgetSize::Fill;
		self
	}

	pub fn fill_height(mut self) -> Self{
		self.height = WidgetSize::Fill;
		self
	}
	
	pub fn fill_width(mut self) -> Self{
		self.height = WidgetSize::Fill;
		self
	}
}

impl From<Size> for IntrinsicSize {
	fn from(value: Size) -> Self {
		IntrinsicSize::new().fixed(value.width as u32, value.height as u32)
	}
}


#[cfg(test)]
mod test{
	use super::*;

	#[test]
	fn test_empty_widgets(){
		let spacing = 12;
		let padding = 12;
		let window = Size::new(800.0, 800.0);
		let mut horizontal_empty_box = WidgetBody::new().layout(
			HorizontalLayout::new(spacing, padding)
		); 
		//Layout::horizontal().spacing(spacing).padding(padding)

		// Make sure spacing and padding don't take effect if widget is empty
		horizontal_empty_box.arrange(window);
		assert_eq!(
			horizontal_empty_box.surface.get_size(),
			Size::new(0.0,0.0),
			"Horizontal empty box does not have zero size"
		);

		let mut vertical_empty_box = WidgetBody::new().layout(
			VerticalLayout::new(spacing, padding)
		);
		//Layout::vertical().spacing(spacing).padding(padding)

		vertical_empty_box.arrange(window);
		assert_eq!(
			vertical_empty_box.surface.get_size(),
			Size::new(0.0,0.0),
			"Vertical empty box does not have zero size"
		);

		let mut block_empty_box = WidgetBody::new().layout(
			BlockLayout::new(spacing)
		);
		block_empty_box.arrange(window);

		assert_eq!(
			block_empty_box.surface.get_size(),
			Size::new(0.0,0.0),
			"Block empty box does not have zero size"
		);
	}

	#[test]
	fn test_horizontal_fit_size(){
		let spacing = 12;
		let padding = 12;
		let window = Size::new(800.0, 800.0);

		let box1 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(200, 150));
		let box2 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(100, 500));
		let box3 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(500, 25));
		let box4 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(300, 20));
		
		let mut horizontal_box = WidgetBody::new()
			.layout(
				HorizontalLayout::new(spacing, padding)
			)
			.add_children(vec![box1,box2,box3,box4])
			.intrinsic_size(IntrinsicSize::new().fit());
		//Layout::horizontal().spacing(spacing).padding(padding)
		
		horizontal_box.arrange(window);

		// The padding should always be added twice for each side, the spacing
		// should be added n-1 times, with n being the number of widgets
		let mut expected_size = Size::new(1100.0, 500.0);
		expected_size.width += (padding * 2) as f32;
		expected_size.width += (spacing * 3) as f32;
		expected_size.height += (padding * 2) as f32;

		assert_eq!(
			horizontal_box.surface.get_size(),
			expected_size,
			"Horizontal fit layout has incorrect size"
		);
	}

	#[test]
	fn test_vertical_fit_size(){
		let spacing = 12;
		let padding = 12;
		let window = Size::new(800.0, 800.0);
		
		let box1 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(200, 200));
		let box2 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(50, 100));
		let box3 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(75, 300));
		let box4 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(10, 400));
		
		let mut vertical_box = WidgetBody::new()
			.layout(
				VerticalLayout::new(spacing, padding)
			)
			.add_children(vec![box1,box2,box3,box4])
			.intrinsic_size(IntrinsicSize::new().fit());
		
		//Layout::vertical().spacing(spacing).padding(padding)

		vertical_box.arrange(window);
		
		// The padding should always be added twice for each side, the spacing
		// should be added n-1 times, with n being the number of widgets
		let mut expected_size = Size::new(200.0, 1000.0);
		expected_size.height += (padding * 2) as f32;
		expected_size.height += (spacing * 3) as f32;
		expected_size.width += (padding * 2) as f32;

		assert_eq!(vertical_box.surface.get_size(),expected_size);
	}

	#[test]
	fn test_block_fit_size(){
		let spacing = 24;
		let padding = 24;
		let window = Size::new(2000.0, 2000.0);
		let box1 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(200, 200));
		
		let mut block_box = WidgetBody::new()
			.layout(BlockLayout::new(padding))
			.add_child(box1)
			.intrinsic_size(IntrinsicSize::new().fit());

		block_box.arrange(window);

		let mut expected_size = Size::new(200.0, 200.0);
		expected_size.width += (padding * 2) as f32;
		expected_size.height += (padding * 2) as f32;
		dbg!(&block_box);
		
		assert_eq!(block_box.surface.get_size(),expected_size);
	}

	#[test]
	fn test_vertical_positioning(){
		let spacing = 24;
		let padding = 56;

		let box1 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(200, 200));
		let box2 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(200, 200));
		let box3 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(200, 200));
		let box4 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(200, 200));

		let mut vertical_box = WidgetBody::new() 
			.layout(VerticalLayout::new(spacing, padding))
			.add_children(vec![box1,box2,box3,box4]);

		vertical_box.arrange(Size::new(800.0, 800.0));

		let mut prev_pos = Position::new(padding as f32, padding as f32);

		// The children should be previous [height + spacing] distance
		// away from each other vertically and have the same x position
		for (i,child) in vertical_box.children.iter().enumerate(){
			let pos = child.surface.get_position();
			let size = child.surface.get_size();

			assert_eq!(prev_pos,pos,"Test failed on iteration: {}",i);

			prev_pos.translate(0.0, size.height);
			prev_pos.translate(0.0, spacing as f32);
		}
	}
	
	#[test]
	fn test_horizontal_positioning(){
		let spacing = 24;
		let padding = 56;

		let box1 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(200, 200));
		let box2 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(200, 200));
		let box3 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(200, 200));
		let box4 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(200, 200));

		let mut horizontal_box = WidgetBody::new()
			.layout(HorizontalLayout::new(spacing, padding))
			.add_children(vec![box1,box2,box3,box4]);

		horizontal_box.arrange(Size::new(800.0, 800.0));

		let mut prev_pos = Position::new(padding as f32, padding as f32);

		// The children should be previous [height + spacing] distance
		// away from each other vertically and have the same x position
		for (i,child) in horizontal_box.children.iter().enumerate(){
			let pos = child.surface.get_position();
			let size = child.surface.get_size();

			assert_eq!(prev_pos,pos,"Test failed on iteration: {}",i);

			prev_pos.translate(size.width, 0.0);
			prev_pos.translate(spacing as f32,0.0);
		}
	}

	#[test]
	fn test_block_positioning(){
		let spacing = 24;
		let padding = 56;

		let box1 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(200, 200));

		let mut block_box = WidgetBody::new()
			.layout(BlockLayout::new(padding))
			.add_children(vec![box1]);

		block_box.arrange(Size::new(800.0, 800.0));

		let parent_pos = block_box.surface.get_position();

		for child in block_box.children.iter(){
			let pos = child.surface.get_position();
			assert_eq!(pos.x,parent_pos.x + padding as f32);
		}
	}

	/// Test the nested horizontal positioning a couple layers deep
	#[test]
	fn test_nested_horizontal_positioning(){
		let window_size = Size::new(800.0, 800.0);
		let spacing = 24;
		let padding = 56;

		let box1 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(200, 200));
		let box2 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(200, 200));
		let box3 = 
			WidgetBody::new()
			.intrinsic_size(IntrinsicSize::new().fixed(200, 200))
			.add_child(box2);

		let mut horizontal_box = 
			WidgetBody::new()
			.layout(HorizontalLayout::new(spacing, padding))
			.add_children(vec![box1,box3]);

		horizontal_box.arrange(window_size);

		let nested_test = |parent:&WidgetBody|{
			let mut prev_pos = parent.surface.get_position();
			prev_pos.translate(padding as f32, padding as f32);
			// The children should be previous [height + spacing] distance
			// away from each other vertically and have the same x position
			for (i,child) in parent.children.iter().enumerate(){
				let pos = child.surface.get_position();
				let size = child.surface.get_size();

				assert_eq!(prev_pos,pos,"Test failed on iteration: {}",i);

				prev_pos.translate(size.width, 0.0);
				prev_pos.translate(spacing as f32,0.0);
				
			}
			
		};

		nested_test(&horizontal_box);
		nested_test(&horizontal_box.children[1]);
	}

	#[test]
	fn test_fill_size(){
		// Add sub-widgets to test nested layouts 
	}
}


