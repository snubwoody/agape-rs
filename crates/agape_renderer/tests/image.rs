use agape_core::Size;
use agape_renderer::image::Image;
use image::ImageBuffer;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;
use tiny_skia::Pixmap;

fn save_image(width: u32, height: u32) -> (TempDir, PathBuf) {
    let mut buf = ImageBuffer::new(width, height);
    for pixel in buf.pixels_mut() {
        *pixel = image::Rgb([0u8, 0u8, 0u8]);
    }
    let id: u64 = rand::random();
    let temp = TempDir::new().unwrap();
    let path = PathBuf::new()
        .join(temp.path())
        .join(format!("img-{id}.png"));
    buf.save(&path).unwrap();
    (temp, path)
}

#[test]
fn inferred_dimensions() {
    let (_temp, path) = save_image(300, 500);
    let data = image::open(path).unwrap();
    let image = Image::new(Arc::new(data));
    assert_eq!(image.size, Size::new(300.0, 500.0));
}

#[test]
fn render_output() {
    let mut pixmap = Pixmap::new(300, 300).unwrap();
    pixmap.fill(tiny_skia::Color::WHITE);
    let (_temp, path) = save_image(300, 500);
    let data = image::open(path).unwrap();
    let image = Image::new(Arc::new(data));
    image.draw(&mut pixmap);
    for pixel in pixmap.pixels() {
        assert_eq!(pixel.red(), 0);
        assert_eq!(pixel.green(), 0);
        assert_eq!(pixel.blue(), 0);
    }
}
