use winit::dpi::PhysicalPosition;

/// The bounds of any object that has a size 
/// and position
pub struct Bounds {
	pub x:[i32;2],
	pub y:[i32;2],
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

//TODO change this to floating point number for better accuracy
/// Represents the position of any structure
pub struct Position{
	pub x:i32,
	pub y:i32
}

impl Position {
	pub fn new(x:i32,y:i32) -> Self {
		Self{x,y}
	}

	/// Translate the position
	pub fn translate(&mut self,x:i32,y:i32) {
		self.x += x;
		self.y += y;
	}

	/// Set the position
	pub fn set(&mut self,x:i32,y:i32){
		self.x = x;
		self.y = y;
	}
}


impl From<PhysicalPosition<f64>> for Position {
	fn from(position: PhysicalPosition<f64>) -> Self {
		Self { 
			x:position.x as i32,
			y:position.y as i32 
		}
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


