use agape_core::{Position, Size};
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use image::RgbaImage;
use tiny_skia::{IntSize, Pixmap, PixmapPaint, Transform};

#[derive(Clone, PartialOrd, PartialEq, Debug, Default)]
pub struct Text {
    pub content: String,
    pub font_size: f32,
    pub position: Position,
}

impl Text {
    /// Create a new [`Text`] primitive, with a default font
    /// size of 16px.
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_owned(),
            font_size: 16.0,
            position: Position::default(),
        }
    }

    pub fn draw_text(
        &self,
        pixmap: &mut Pixmap,
        text_size: Size,
        font_system: &mut FontSystem,
        cache: &mut SwashCache,
    ) {
        // TODO: cache text that has the same content and font size
        let metrics = Metrics::new(self.font_size, self.font_size);
        let mut buffer = Buffer::new(font_system, metrics);
        let mut buffer = buffer.borrow_with(font_system);
        let attrs = Attrs::new();
        buffer.set_text(&self.content, &attrs, Shaping::Advanced);
        buffer.shape_until_scroll(true);

        // TODO: add clippy lint for conversion
        let text_color = cosmic_text::Color::rgb(0, 0, 0);
        let width = text_size.width.ceil() as u32;
        let height = text_size.width.ceil() as u32;

        let mut image = RgbaImage::new(width, height);
        let size = IntSize::from_wh(image.width(), image.height()).unwrap();
        buffer.draw(cache, text_color, |x, y, _, _, color| {
            let [r, g, b, a] = color.as_rgba();
            image.put_pixel(x.max(0) as u32, y as u32, image::Rgba([r, g, b, a]));
        });

        let glyph_pixmap = Pixmap::from_vec(image.to_vec(), size).unwrap();
        let Position { x, y } = self.position;
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
