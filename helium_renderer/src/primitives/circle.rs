use helium_core::{Position,Color};
use super::{IntoPrimitive, Primitive};

#[derive(Debug,Clone,Copy,PartialEq, PartialOrd,Default)]
pub struct Circle{
	pub diameter:f32,
	pub position:Position,
	pub color:Color,
}

impl Circle {
	pub fn new(diameter:f32) -> Self{
		Self{
			diameter,
			position:Position::default(),
			color:Color::default()
		}
	}

	pub fn unit(value:f32) -> Self{
		Self{
			diameter:value,
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

impl IntoPrimitive for Circle {
	fn into_primitive(self) -> Primitive{
		Primitive::Circle(self)
	}
}