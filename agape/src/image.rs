use image::{DynamicImage, ImageReader};

/// Load an image.
pub fn load_image(path: &str) -> crate::Result<DynamicImage> {
    let image = ImageReader::open(path)?.decode()?;
    Ok(image)
}
