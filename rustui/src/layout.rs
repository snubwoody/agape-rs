use crate::widgets::WidgetBody;

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

// TODO maybe have a constraint struct then add 
// width and height and give each individual item
// an instrinsic size
#[derive(Debug,Clone, Copy)]
pub enum IntrinsicSize {
	Fill{width:bool,height:bool},
	FillWidth,
	FillHeight,
	Fit{padding:u32},
	Flex,
	Fixed(f32,f32),
}
