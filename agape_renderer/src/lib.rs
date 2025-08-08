use agape_core::Position;
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use image::RgbaImage;
use std::path::Path;
use tiny_skia::{IntSize, Pixmap, PixmapPaint, Transform};
// TODO: mention that only ttf/otf fonts are supported

pub struct Renderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
}

impl Renderer {
    /// Create a new [`Renderer`].
    pub fn new() -> Self {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        Self {
            font_system,
            swash_cache,
        }
    }

    /// Load a font into the font database.
    pub fn load_font_data(&mut self, data: Vec<u8>) {
        self.font_system.db_mut().load_font_data(data)
    }

    pub fn load_font_source(&mut self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        self.font_system.db_mut().load_font_file(path)
    }

    pub fn load_fonts_dir(&mut self, path: impl AsRef<Path>) {
        self.font_system.db_mut().load_fonts_dir(path)
    }

    /// Draw text onto the `Pixmap`.
    pub fn draw_text(&self, pixmap: &mut Pixmap, text: &str, font_size: f32, position: Position) {
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
}

#[cfg(test)]
mod test {
    use super::*;
}
