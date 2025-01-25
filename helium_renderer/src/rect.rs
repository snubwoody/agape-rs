use helium_core::{Size,Position,Color};

#[derive(Debug,Clone)]
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

	pub fn position(mut self,x:f32,y:f32) -> Self{
		self.position = Position{x,y};
		self
	}

	pub fn color(mut self,color:Color) -> Self{
		self.color = color;
		self
	}
}