use agape_core::{Border, Color, Position, Rgba, Size, map};
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use image::{DynamicImage, GenericImageView, RgbaImage};
use std::path::Path;
use tiny_skia::{IntSize, Paint, PathBuilder, Pixmap, PixmapPaint, Stroke, Transform};
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
    pub fn draw_image(
        &mut self,
        pixmap: &mut Pixmap,
        image: &DynamicImage,
        position: Position,
        size: Size,
    ) {
        let (width, height) = image.dimensions();
        let data = image.to_rgba8().into_raw();
        let pixmap_size = IntSize::from_wh(width, height).unwrap();
        let image_pixmap = Pixmap::from_vec(data, pixmap_size).unwrap();

        let scale_x = size.width / image_pixmap.width() as f32;
        let scale_y = size.height / image_pixmap.height() as f32;
        let transform =
            Transform::from_translate(position.x, position.y).post_scale(scale_x, scale_y);

        let x = position.x as i32;
        let y = position.y as i32;
        let paint = PixmapPaint::default();
        pixmap.draw_pixmap(x, y, image_pixmap.as_ref(), &paint, transform, None);
    }

    /// Draw a rectangle onto the `Pixmap`.
    pub fn draw_rect(
        &mut self,
        pixmap: &mut Pixmap,
        color: &Color<Rgba>,
        size: Size,
        position: Position,
        border: Option<Border>,
    ) {
        let (r, g, b, a) = color.inner();

        // Map the alpha since it's clipped to 100
        let a = map(a as f32, [0.0, 100.0], [0.0, 255.0]) as u8;
        let mut paint = Paint::default();
        paint.set_color_rgba8(r, g, b, a);

        let Position { x, y } = position;
        let Size { width, height } = size;

        let rect = tiny_skia::Rect::from_xywh(x, y, width, height).unwrap();
        pixmap.fill_rect(rect, &paint, Transform::identity(), None);

        if let Some(border) = border {
            // TODO turn this into a function
            let (r, g, b, a) = border.color.inner();
            let a = map(a as f32, [0.0, 100.0], [0.0, 255.0]) as u8;

            let mut border_paint = Paint::default();
            border_paint.set_color_rgba8(r, g, b, a);
            let mut path_builder = PathBuilder::new();
            path_builder.push_rect(rect);
            let path = path_builder.finish().unwrap();

            let stroke = Stroke {
                width: border.width,
                ..Default::default()
            };

            pixmap.stroke_path(&path, &border_paint, &stroke, Transform::identity(), None);
        }
    }

    /// Draw text onto the `Pixmap`.
    pub fn draw_text(
        &mut self,
        pixmap: &mut Pixmap,
        text: &str,
        font_size: f32,
        position: Position,
    ) {
        let text_size = self.text_size(text, font_size);

        let font_system = &mut self.font_system;
        let swash_cache = &mut self.swash_cache;
        let metrics = Metrics::new(font_size, font_size);
        let mut buffer = Buffer::new(font_system, metrics);
        let mut buffer = buffer.borrow_with(font_system);
        let attrs = Attrs::new();
        buffer.set_text(text, &attrs, Shaping::Advanced);
        buffer.shape_until_scroll(true);

        // TODO: add clippy lint for conversion
        let text_color = cosmic_text::Color::rgb(0, 0, 0);
        let width = text_size.width.ceil() as u32;
        let height = text_size.width.ceil() as u32;
        let mut image = RgbaImage::new(width, height);
        let size = IntSize::from_wh(image.width(), image.height()).unwrap();
        buffer.draw(swash_cache, text_color, |x, y, _, _, color| {
            let [r, g, b, a] = color.as_rgba();
            image.put_pixel(x.max(0) as u32, y as u32, image::Rgba([r, g, b, a]));
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
