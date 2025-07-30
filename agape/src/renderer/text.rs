use crate::FONT;
use agape_core::{Position, Size};
use tiny_skia::{IntSize, Pixmap, PixmapPaint, Transform};

/// Draw text onto the `Pixmap`.
pub fn draw_text(pixmap: &mut Pixmap, text: &str, font_size: f32, position: Position) {
    let font = FONT.get().unwrap();

    let mut x_pos: i32 = position.x as i32;
    let y_pos: i32 = position.y as i32;

    let font_metrics = font.horizontal_line_metrics(font_size).unwrap();
    let ascent = font_metrics.ascent.round() as i32;

    // Draw each character onto a pixmap then
    // draw that pixmap onto the root pixmap
    for c in text.chars() {
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

        // TODO add custom colors
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
