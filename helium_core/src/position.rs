use winit::dpi::{PhysicalPosition, PhysicalSize};

/// Represents the position of any structure
#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Default)]
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


/// The bounds of any object that has a [`Size`] 
/// and [`Position`].
#[derive(Debug,Clone,Copy,PartialEq, PartialOrd,Default)]
pub struct Bounds {
	pub x:[f32;2], // TODO change this to start and end position and add a center, and make private
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