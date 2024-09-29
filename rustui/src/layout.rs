use crate::widgets::WidgetTree;

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

#[derive(Debug,Clone, Copy)]
pub enum WidgetSize{
	Fixed(f32),
	Fill,
	Fit(f32)
}
// TODO maybe have a constraint struct then add 
// width and height and give each individual item
// an instrinsic size
#[derive(Debug,Clone,Copy)]
pub struct IntrinsicSize {
	pub width:WidgetSize,
	pub height:WidgetSize
}

