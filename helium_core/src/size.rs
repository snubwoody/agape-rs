
use std::ops::{Add, AddAssign, Sub};

use winit::dpi::PhysicalSize;

/// Anything with a width and a height
// TODO change size and position to u32
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