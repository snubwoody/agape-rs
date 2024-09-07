use winit::dpi::PhysicalPosition;

/// The bounds of any object that has a size 
/// and position
pub struct Bounds {
	pub x:[f32;2],
	pub y:[f32;2],
}

impl Bounds{
	/// Check if a [`Position`] is within the bounds
	pub fn within(&self,position:&Position) -> bool {
		if 
			position.x > self.x[0] && 
			position.x < self.x[1] &&
			position.y > self.y[0] &&
			position.y < self.y[1] {
			return true;
		}

		false
	}
}

/// Represents the position of any structure
#[derive(Debug,Clone,Copy,PartialEq,PartialOrd)]
pub struct Position{
	pub x:f32,
	pub y:f32,
}

impl Position {
	pub fn new(x:f32,y:f32) -> Self {
		Self{x,y}
	}

	/// Translate the position
	pub fn translate(&mut self,x:f32,y:f32) {
		self.x += x;
		self.y += y;
	}

	/// Set the position
	pub fn set(&mut self,x:f32,y:f32){
		self.x = x;
		self.y = y;
	}
}


impl From<PhysicalPosition<f64>> for Position {
	fn from(position: PhysicalPosition<f64>) -> Self {
		Self { 
			x:position.x as f32,
			y:position.y as f32 
		}
	}
}


// TODO
pub struct Size{
	width:f32,
	height:f32
}

impl Size {
	pub fn scale(&mut self,factor:f32) {
		self.width *= factor;
		self.height *= factor;
	}

	pub fn set(&mut self,width:f32,height:f32) {
		self.width = width;
		self.height = height;
	}
}


/// Map value from one range to another. Any overflow is clipped to the min or max
pub fn map(mut value:f32,input_range:[f32;2],output_range:[f32;2]) -> f32{
	if value > input_range[1]{
		value = input_range[1]
	}
	else if value < input_range[0] {
		value = input_range[0]
	}

	let scale = (output_range[1] - output_range[0]) / (input_range[1] - input_range[0]);
	let offset = input_range[0]*(scale)+output_range[0];

	return  value * scale + offset;
}


