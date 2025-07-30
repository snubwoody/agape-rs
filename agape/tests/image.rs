use image::ImageBuffer;
use std::fs;
use std::path::PathBuf;

fn save_image() {
    let mut buf = ImageBuffer::new(250, 250);
    for pixel in buf.pixels_mut() {
        *pixel = image::Rgb([0, 0, 0]);
    }
    let _ = fs::create_dir("temp");
    let id: u64 = rand::random();
    let path = PathBuf::from(format!("temp/img-{}.png", id));
}

#[test]
fn supported_image_formats() {
    save_image();
}
