pub mod image;
pub mod rect;
pub mod svg;
pub mod text;

pub use crate::image::Image;
use crate::rect::Rect;
pub use crate::text::Text;
use agape_core::Size;
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use std::path::Path;
pub use svg::Svg;
use tiny_skia::Pixmap;

// TODO: mention that only ttf/otf fonts are supported

pub struct Renderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
    pixmap: Pixmap,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    /// Create a new [`Renderer`].
    pub fn new() -> Self {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let pixmap = Pixmap::new(500, 500).unwrap();

        Self {
            font_system,
            swash_cache,
            pixmap,
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

    /// Resize the `Pixmap`. Does nothing if the `width` or `height` is 0.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.pixmap = Pixmap::new(width, height).unwrap();
        }
    }

    /// Get a reference to the `Pixmap`.
    pub fn pixmap(&self) -> &Pixmap {
        &self.pixmap
    }

    /// Get a `&mut` to the `Pixmap`.
    pub fn pixmap_mut(&mut self) -> &mut Pixmap {
        &mut self.pixmap
    }

    /// Draw an svg onto the pixmap.
    pub fn draw_svg(&mut self, svg: Svg) {
        svg.draw(&mut self.pixmap);
    }

    /// Draw an image onto the pixmap.
    pub fn draw_image(&mut self, image: Image) {
        image.draw(&mut self.pixmap);
    }

    /// Draw a rectangle onto the `Pixmap`.
    pub fn draw_rect(&mut self, rect: Rect) {
        rect.draw(&mut self.pixmap);
    }

    /// Draw text onto the `Pixmap`.
    pub fn draw_text(&mut self, text: Text) {
        let size = self.text_size(&text.content, text.font_size);
        text.draw_text(
            &mut self.pixmap,
            size,
            &mut self.font_system,
            &mut self.swash_cache,
        )
    }

    /// Get the text size.
    pub fn text_size(&mut self, text: &str, font_size: f32) -> Size {
        let font_system = &mut self.font_system;
        let metrics = Metrics::new(font_size, font_size);
        let mut buffer = Buffer::new(font_system, metrics);
        let mut buffer = buffer.borrow_with(font_system);

        let attrs = Attrs::new();
        buffer.set_text(text, &attrs, Shaping::Advanced);
        buffer.shape_until_scroll(true);

        let mut width = 0.0;
        let mut height = 0.0;

        for run in buffer.layout_runs() {
            width += run.line_w;
            height += run.line_height;
        }

        Size::new(width, height)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn text_size() {
        let mut renderer = Renderer::new();
        let size = renderer.text_size("Hello world", 16.0);
        assert!(size.height >= 16.0);
        assert!(size.width >= 16.0);
    }
}
