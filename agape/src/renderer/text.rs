use crate::FONT;
use agape_core::{Position, Size};
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use image::RgbaImage;
use tiny_skia::{IntSize, Pixmap, PixmapPaint, Transform};

/// Draw text onto the `Pixmap`.
pub fn draw_text(pixmap: &mut Pixmap, text: &str, font_size: f32, position: Position) {
    let mut font_system = FontSystem::new();
    let mut swash_cache = SwashCache::new();
    let metrics = Metrics::new(font_size, font_size);
    let mut buffer = Buffer::new(&mut font_system, metrics);
    let mut buffer = buffer.borrow_with(&mut font_system);
    let attrs = Attrs::new();
    buffer.set_text(text, &attrs, Shaping::Advanced);
    buffer.shape_until_scroll(true);

    let text_color = cosmic_text::Color::rgb(0, 0, 0);
    let mut image = RgbaImage::new(200, 200);
    let size = IntSize::from_wh(image.width(), image.height()).unwrap();
    buffer.draw(&mut swash_cache, text_color, |x, y, _, _, color| {
        let [r, g, b, a] = color.as_rgba();
        image.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a]));
    });
    let glyph_pixmap = Pixmap::from_vec(image.to_vec(), size).unwrap();
    let Position { x, y } = position;
    pixmap.draw_pixmap(
        x as i32,
        y as i32,
        glyph_pixmap.as_ref(),
        &PixmapPaint::default(),
        Transform::identity(),
        None,
    );
}

/// Get the total size of a string of text
pub fn text_size(text: &str, font_size: f32) -> Size {
    let mut width = 0.0;
    let mut height = 0.0;

    for c in text.chars() {
        let metrics = FONT.get().unwrap().metrics(c, font_size);
        width += metrics.advance_width;
        if metrics.height as f32 > height {
            height = metrics.height as f32;
        }
    }

    Size::new(width, height)
}
