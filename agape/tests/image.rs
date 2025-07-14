use agape::image::load_image;
use agape_core::{IntoColor, Size};
use image::{DynamicImage, ImageBuffer, Rgba};
use rand::random;
use std::fs;
use std::path::PathBuf;

fn save_image(size: Size, color: impl IntoColor<agape_core::Rgba>) -> agape::Result<PathBuf> {
    let color = color.into_color();
    let (r, g, b, _) = color.inner();

    if fs::read_dir("temp").is_err() {
        fs::create_dir("temp")?;
    }

    let id: i64 = random();
    let file_name = format!("image-{id}");
    let path = PathBuf::from(&format!("temp/{file_name}.png"));

    let img_buffer =
        ImageBuffer::from_pixel(size.width as u32, size.height as u32, Rgba([r, g, b, 255]));
    let img = DynamicImage::ImageRgba8(img_buffer);
    img.save(&path)?;

    Ok(path)
}

#[test]
fn load_png() -> agape::Result<()> {
    let path = save_image(Size::unit(100.0), 100)?;
    let image = load_image(&path.to_str().unwrap())?;

    assert_eq!(image.width(), 100);
    Ok(())
}
