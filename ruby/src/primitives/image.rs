use helium_core::{Position, Size};
use image::{ImageBuffer, Rgba};

pub struct Image{
	pub size: Size,
	// TODO make this private and expose using method?
	pub(crate) data: ImageBuffer<Rgba<u8>,Vec<u8>>,
	pub position: Position
}

impl Image {
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

impl std::fmt::Debug for Image{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Image")
			.field("size", &self.size)
			.field("position", &self.position)
			.field("data", &"ImageBuffer<...>")
			.finish()
	}
}


#[cfg(test)]
mod tests{
    use super::*;

	#[test]
	fn image_min_size(){
		let data = image::DynamicImage::new(0, 0, image::ColorType::Rgba16);
		let image = Image::new(data.into())
			.size(-1.0, -1.0);

		assert_eq!(image.size,Size::unit(1.0));
	}
}