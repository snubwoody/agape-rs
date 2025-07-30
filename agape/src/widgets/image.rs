use crate::Error::UnsupportedImageFormat;
use image::{DynamicImage, ImageFormat, ImageReader};
use std::path::Path;

// TODO: re-export image format
pub struct Image {
    data: DynamicImage,
}

impl Image {
    pub fn open<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let reader = ImageReader::open(path)?.with_guessed_format()?;
        if let Some(format) = reader.format() {
            match format {
                ImageFormat::Png | ImageFormat::Jpeg | ImageFormat::WebP => (),
                _ => return Err(UnsupportedImageFormat),
            }
        } else {
            return Err(UnsupportedImageFormat);
        }

        let image = reader.decode()?;

        Ok(Self { data: image })
    }
}
