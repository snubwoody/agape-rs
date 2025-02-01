use super::{IntoPrimitive, Primitive};
use helium_core::{Position,Size};

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub image: ::image::DynamicImage,
	pub size: Size,
    pub position: Position,
}

impl Image {
    pub fn new(image: ::image::DynamicImage) -> Self {
        Self {
            image,
			size:Size::unit(1.0),
            position: Position::default(),
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = Position { x, y };
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = Size { 
			width:width.max(1.0), // Sizes of 0 crash wgpu 
			height:height.max(1.0) 
		};
        self
    }
}

impl IntoPrimitive for Image {
    fn into_primitive(self) -> Primitive {
        Primitive::Image(self)
    }
}
