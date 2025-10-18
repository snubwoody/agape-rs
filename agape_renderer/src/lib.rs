pub mod image;
pub mod rect;
pub mod svg;
pub mod text;

pub use crate::image::Image;
use crate::rect::Rect;
pub use crate::text::{FontQuery, Text};
use agape_core::Size;
use cosmic_text::fontdb::Database;
pub use cosmic_text::{Family, Style, Weight};
use cosmic_text::{FontSystem, SwashCache};
use std::path::Path;
pub use svg::Svg;
use tiny_skia::Pixmap;

// TODO: mention that only ttf/otf fonts are supported

/// 2D renderer.
pub struct Renderer {
    font_system: FontSystem,
    /// The [`SwashCache`] stores rasterised glyphs.
    swash_cache: SwashCache,
    pixmap: Pixmap,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    /// Create a new [`Renderer`], creating a renderer is fairly expensive
    /// as such it is advised to have one per application.
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

    pub fn db(&self) -> &Database {
        self.font_system.db()
    }

    pub fn db_mut(&mut self) -> &mut Database {
        self.font_system.db_mut()
    }

    /// Load a font file into the font system.
    pub fn load_font_file(&mut self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        self.font_system.db_mut().load_font_file(path)
    }

    /// Load all fonts in a directory into the font system,
    /// malformed font files will be skipped.
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
        text.draw_text(
            &mut self.pixmap,
            &mut self.font_system,
            &mut self.swash_cache,
        )
    }

    /// Get the text size.
    pub fn text_size(&mut self, text: Text) -> Size {
        text.size(&mut self.font_system)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn text_size() {
        let mut renderer = Renderer::new();
        let text = Text::new("Hello world").font_size(16.0);
        let size = renderer.text_size(text);
        assert!(size.height >= 16.0);
        assert!(size.width >= 16.0);
    }
}
