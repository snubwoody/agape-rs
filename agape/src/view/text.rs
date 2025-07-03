use super::View;
use agape_core::{Color, GlobalId, Position, Rgba, Size};
use fontdue::Font;
use tiny_skia::{IntSize, Pixmap, PixmapPaint, Transform};

pub struct TextView {
    id: GlobalId,
    position: Position,
    size: Size,
    foreground_color: Color<Rgba>,
    text: String,
    font: Font,
    pub font_size: u8,
}

impl Default for TextView {
    fn default() -> TextView {
        TextView::new("")
    }
}

// TODO add background color
impl TextView {
    pub fn new(text: &str) -> Self {
        let bytes = include_bytes!("../../fonts/Inter/static/Inter-Regular.ttf") as &[u8];
        let font = Font::from_bytes(bytes, fontdue::FontSettings::default()).unwrap();

        Self {
            id: GlobalId::new(),
            position: Position::default(),
            size: Size::default(),
            foreground_color: Color::BLACK,
            text: text.to_owned(),
            font,
            font_size: 16,
        }
    }

    /// Get the total size of a string of text
    pub fn text_size(&self) -> Size {
        let font_size = self.font_size as f32;

        let mut width = 0.0;
        let mut height = 0.0;
        for c in self.text.chars() {
            let metrics = self.font.metrics(c, font_size);
            width += metrics.advance_width;
            if metrics.height as f32 > height {
                height = metrics.height as f32;
            }
        }

        Size::new(width, height)
    }
}

impl View for TextView {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn color(&self) -> &Color<Rgba> {
        &self.foreground_color
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_size(&mut self, size: Size) {
        self.size = size
    }

    fn set_id(&mut self, id: GlobalId) {
        self.id = id
    }

    fn render(&self, pixmap: &mut Pixmap) {
        // pixmap.fill(tiny_skia::Color::WHITE); //  FIXME remove this
        let bytes = include_bytes!("../../fonts/Inter/static/Inter-Regular.ttf") as &[u8];
        let font = Font::from_bytes(bytes, fontdue::FontSettings::default()).unwrap();
        let font_size = 16.0;

        let mut x_pos: i32 = self.position.x as i32;
        let y_pos: i32 = self.position.y as i32;

        let font_metrics = font.horizontal_line_metrics(font_size).unwrap();
        let ascent = font_metrics.ascent.round() as i32;

        // Draw each character onto a pixmap then
        // draw that pixmap onto the root pixmap
        for c in self.text.chars() {
            let (metrics, bitmap) = font.rasterize(c, font_size);

            // Skip spaces to avoid panicking
            if metrics.width == 0 {
                x_pos += metrics.advance_width as i32;
                continue;
            }

            // This will make every character lie on the baseline
            let y_pos: i32 = y_pos + ascent - metrics.height as i32;
            x_pos += metrics.xmin;
            let size = IntSize::from_wh(metrics.width as u32, metrics.height as u32).unwrap();

            let mut colors = vec![];

            for a in bitmap.iter() {
                colors.push(0);
                colors.push(0);
                colors.push(0);
                colors.push(*a);
            }

            let glyph_pixmap = Pixmap::from_vec(colors, size).unwrap();
            let paint = PixmapPaint::default();

            pixmap.draw_pixmap(
                x_pos,
                y_pos,
                glyph_pixmap.as_ref(),
                &paint,
                Transform::default(),
                None,
            );

            // Move the cursor to the next character
            // This is essentially letter spacing
            x_pos += metrics.advance_width.round() as i32;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn text_size() {
        let view = TextView::new("he");
        let size = view.text_size();
    }

    #[test]
    fn text_rendering() {
        let view = TextView::new("Hello world!");
        let mut pixmap = Pixmap::new(500, 500).unwrap();
        view.render(&mut pixmap);
    }
}
