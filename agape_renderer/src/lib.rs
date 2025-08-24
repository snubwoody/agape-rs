pub mod image;
pub mod rect;
pub mod text;

pub use crate::image::Image;
use crate::rect::Rect;
pub use crate::text::Text;
use agape_core::{Position, Size};
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use std::path::Path;
use tiny_skia::{Pixmap, Transform};
use usvg::Tree;
// TODO: mention that only ttf/otf fonts are supported

pub struct Renderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
}

// TODO: text size
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

    pub fn draw_svg(&mut self, pixmap: &mut Pixmap, tree: &Tree, position: Position, size: Size) {
        let svg_width = tree.size().width();
        let svg_height = tree.size().height();
        let scale_x = size.width / svg_width;
        let scale_y = size.height / svg_height;
        let transform =
            Transform::from_translate(position.x, position.y).post_scale(scale_x, scale_y);

        resvg::render(tree, transform, &mut pixmap.as_mut());
    }

    /// Draw an image onto the pixmap.
    pub fn draw_image(&mut self, pixmap: &mut Pixmap, image: Image) {
        image.draw(pixmap);
    }

    /// Draw a rectangle onto the `Pixmap`.
    pub fn draw_rect(&mut self, pixmap: &mut Pixmap, rect: Rect) {
        rect.draw(pixmap);
    }

    /// Draw text onto the `Pixmap`.
    pub fn draw_text(&mut self, pixmap: &mut Pixmap, text: Text) {
        let size = self.text_size(&text.content, text.font_size);
        text.draw_text(pixmap, size, &mut self.font_system, &mut self.swash_cache)
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
