use helium_core::{Size,Position,Color};

use super::{IntoPrimitive, Primitive};

#[derive(Debug,Clone,Copy,PartialEq, PartialOrd,Default)]
pub struct Rect{
	pub size:Size,
	pub position:Position,
	pub color:Color,
}

impl Rect {
	pub fn new(width:f32,height:f32) -> Self{
		Self{
			size:Size{width,height},
			position:Position::default(),
			color:Color::default()
		}
	}

	pub fn unit(value:f32) -> Self{
		Self{
			size:Size::unit(value),
			..Default::default()
		}
	}

	pub fn position(mut self,x:f32,y:f32) -> Self{
		self.position = Position{x,y};
		self
	}

	pub fn color(mut self,color:Color) -> Self{
		self.color = color;
		self
	}
}

impl IntoPrimitive for Rect {
	fn into_primitive(self) -> Primitive{
		Primitive::Rect(self)
	}
}