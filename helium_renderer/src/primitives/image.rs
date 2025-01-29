use super::{IntoPrimitive, Primitive};
use helium_core::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub image: ::image::DynamicImage,
    pub position: Position,
}

impl Image {
    pub fn new(image: ::image::DynamicImage) -> Self {
        Self {
            image,
            position: Position::default(),
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = Position { x, y };
        self
    }
}

impl IntoPrimitive for Image {
    fn into_primitive(self) -> Primitive {
        Primitive::Image(self)
    }
}
