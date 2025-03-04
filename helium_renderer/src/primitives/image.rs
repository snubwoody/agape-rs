use super::{IntoSurface, Surface};
use helium_core::{Position, Size};
use image::{ImageBuffer, Rgba};

#[derive(Clone, PartialEq)]
pub struct ImageSurface {
    pub size: Size,
    pub data: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub position: Position,
}

impl ImageSurface {
    pub fn new(data: ::image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> Self {
        // TODO get the size of the image and make it a u8 max?
        let width = data.width();
        let height = data.height();
        let size = Size {
            width: width as f32,
            height: height as f32,
        };
        Self {
            data,
            size,
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

impl std::fmt::Debug for ImageSurface{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ImageSurface")
			.field("size", &self.size)
			.field("position", &self.position)
			.field("data", &"ImageBuffer<...>")
			.finish()
	}
}

impl IntoSurface for ImageSurface {
    fn into_surface(self) -> Surface {
        Surface::Image(self)
    }
}
