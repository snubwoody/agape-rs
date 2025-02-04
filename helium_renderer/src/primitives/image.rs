use super::{IntoPrimitive, Primitive};
use helium_core::{Position, Size};
use image::{ImageBuffer, Rgba};

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub size: Size,
    pub data: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub position: Position,
}

impl Image {
    pub fn new(data: ::image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> Self {
        Self {
            data,
            size: Size::unit(1.0),
            position: Position::default(),
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = Position { x, y };
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = Size {
            width: width.max(1.0), // Sizes of 0 crash wgpu
            height: height.max(1.0),
        };
        self
    }
}

impl IntoPrimitive for Image {
    fn into_primitive(self) -> Primitive {
        Primitive::Image(self)
    }
}
