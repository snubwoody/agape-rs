use agape_core::{Position, Size};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use tiny_skia::{IntSize, Pixmap, PixmapPaint, Transform};

/// Draw an image onto the pixmap.
pub fn draw_image(pixmap: &mut Pixmap, image: &DynamicImage, position: Position, size: Size) {
    // TODO: use resize exact?
    let (width, height) = size.to_u32();
    let data = image
        .resize(width, height, FilterType::CatmullRom)
        .to_rgba8()
        .into_raw();
    let size = IntSize::from_wh(width, height).unwrap();
    let image_pixmap = Pixmap::from_vec(data, size).unwrap();

    let x = position.x as i32;
    let y = position.y as i32;
    let paint = PixmapPaint::default();
    pixmap.draw_pixmap(
        x,
        y,
        image_pixmap.as_ref(),
        &paint,
        Transform::identity(),
        None,
    );
}
