use super::{IntoPrimitive, Primitive};
use helium_core::{color::BLACK, Color, Position};

#[derive(Debug, Clone, PartialEq)]
pub struct Icon {
    pub image: ::image::DynamicImage,
    pub position: Position,
    pub color: Color,
}

impl Icon {
    pub fn new(image: ::image::DynamicImage) -> Self {
        Self {
            image,
            position: Position::default(),
            color: BLACK,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = Position { x, y };
        self
    }
}

impl IntoPrimitive for Icon {
    fn into_primitive(self) -> Primitive {
        Primitive::Icon(self)
    }
}
