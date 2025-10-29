use agape::widgets::Image;
use agape_core::Size;
use image::{ImageBuffer, ImageFormat};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn save_image(width: u32, height: u32, format: ImageFormat) -> (TempDir, PathBuf) {
    let ext = format.extensions_str()[0];
    let mut buf = ImageBuffer::new(width, height);
    for pixel in buf.pixels_mut() {
        *pixel = image::Rgb([0u8, 0u8, 0u8]);
    }
    let id: u64 = rand::random();
    let temp = TempDir::new().unwrap();
    let path = PathBuf::new()
        .join(temp.path())
        .join(format!("img-{id}.{ext}"));
    buf.save(&path).unwrap();
    (temp, path)
}

#[test]
fn inferred_dimensions() {
    let (_temp, path) = save_image(300, 500, ImageFormat::WebP);
    let webp = Image::open(path).unwrap();

    assert_eq!(webp.dimensions(), Size::new(300.0, 500.0));
}

#[test]
fn supported_image_formats() {
    let (_temp, png_path) = save_image(10, 10, ImageFormat::Png);
    let (_temp, jpeg_path) = save_image(20, 20, ImageFormat::Jpeg);
    let (_temp, webp_path) = save_image(30, 30, ImageFormat::WebP);

    let png = Image::open(png_path).unwrap();
    let jpeg = Image::open(jpeg_path).unwrap();
    let webp = Image::open(webp_path).unwrap();

    assert_eq!(png.dimensions(), Size::unit(10.0));
    assert_eq!(jpeg.dimensions(), Size::unit(20.0));
    assert_eq!(webp.dimensions(), Size::unit(30.0));
}

#[test]
fn load_from_memory() {
    let (_temp, path) = save_image(10, 10, ImageFormat::Png);
    let bytes = fs::read(path).unwrap();

    let png = Image::bytes(&bytes).unwrap();

    assert_eq!(png.dimensions(), Size::unit(10.0));
}

#[test]
fn unsupported_image_formats() {
    let (_temp, avif_path) = save_image(30, 30, ImageFormat::Avif);
    let result = Image::open(avif_path);

    assert!(matches!(result, Err(agape::Error::UnsupportedImageFormat)));
}
