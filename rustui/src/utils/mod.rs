use std::ops::{Add, AddAssign, Sub};
use winit::dpi::{PhysicalPosition, PhysicalSize};

/// The bounds of any object that has a [`Size`] 
/// and [`Position`].
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

//TODO change this to a generic
impl From<PhysicalPosition<f64>> for Position {
	fn from(position: PhysicalPosition<f64>) -> Self {
		Self { 
			x:position.x as f32,
			y:position.y as f32 
		}
	}
}


#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Default)]
pub struct Size{
	pub width:f32,
	pub height:f32
}

impl Size {
	pub fn new(width:f32,height:f32) -> Self {
		Self { width, height }
	}
	
	pub fn scale(&mut self,factor:f32) {
		self.width *= factor;
		self.height *= factor;
	}

	pub fn set(&mut self,width:f32,height:f32) {
		self.width = width;
		self.height = height;
	}
}

impl From<PhysicalSize<u32>> for Size {
	fn from(size: PhysicalSize<u32>) -> Self {
		Self { 
			width: size.width as f32, 
			height: size.height as f32 
		}
	}
}


impl AddAssign for Size {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            width: self.width + other.width,
            height: self.height + other.height,
        };
    }
}

impl Add for Size {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self{
			width:self.width + rhs.width,
			height:self.height + rhs.height
		}
	}
}

impl Sub for Size {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self{
			width:self.width - rhs.width,
			height:self.height - rhs.height
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


