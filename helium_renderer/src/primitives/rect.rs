use super::{IntoPrimitive, Primitive};
use helium_core::{Color, Position, Size};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Rect {
    pub size: Size,
    pub position: Position,
    pub color: Color,
    pub corner_radius: f32,
}

impl Rect {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: Size { width, height },
            ..Default::default()
        }
    }

    pub fn unit(value: f32) -> Self {
        Self {
            size: Size::unit(value),
            ..Default::default()
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = Position { x, y };
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn corner_radius(mut self, corner_radius: f32) -> Self {
        self.corner_radius = corner_radius;
        self
    }
}

impl IntoPrimitive for Rect {
    fn into_primitive(self) -> Primitive {
        Primitive::Rect(self)
    }
}

impl From<&dyn crystal::Layout> for Rect {
	fn from(layout: &dyn crystal::Layout) -> Self {
		let size = layout.size(); 
		let position = layout.position(); 
		
		Self::new(size.width, size.height)
			.position(position.x, position.y)
	}
}
