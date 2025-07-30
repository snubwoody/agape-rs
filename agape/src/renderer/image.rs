use agape_core::{Position, Size};
use image::{DynamicImage, GenericImageView};
use tiny_skia::{IntSize, Pixmap, PixmapPaint, Transform};

/// Draw an image onto the pixmap.
pub fn draw_image(pixmap: &mut Pixmap, image: &DynamicImage, position: Position, size: Size) {
    let (width, height) = image.dimensions();
    let data = image.to_rgba8().into_raw();
    let pixmap_size = IntSize::from_wh(width, height).unwrap();
    let image_pixmap = Pixmap::from_vec(data, pixmap_size).unwrap();

    let scale_x = size.width / image_pixmap.width() as f32;
    let scale_y = size.height / image_pixmap.height() as f32;
    let transform = Transform::from_translate(position.x, position.y).post_scale(scale_x, scale_y);

    let x = position.x as i32;
    let y = position.y as i32;
    let paint = PixmapPaint::default();
    pixmap.draw_pixmap(x, y, image_pixmap.as_ref(), &paint, transform, None);
}
