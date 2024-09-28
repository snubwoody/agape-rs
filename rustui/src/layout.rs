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
#[derive(Debug,Clone,Copy)]
pub enum IntrinsicSize {
	Fill{width:bool,height:bool},
	Fit{padding:u32},
	Fixed(f32,f32),
}

pub struct LayoutManager{
	size:IntrinsicSize,
	layout:Layout
}

impl LayoutManager {
	
}
