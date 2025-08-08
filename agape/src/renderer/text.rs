use crate::FONT;
use agape_core::{Position, Size};
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use image::RgbaImage;
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

fn render_text() {
    let mut pixmap = Pixmap::new(200, 200).unwrap();
    let mut font_system = FontSystem::new();
    let mut swash_cache = SwashCache::new();
    let metrics = Metrics::new(16.0, 16.0);
    let mut buffer = Buffer::new(&mut font_system, metrics);
    let mut buffer = buffer.borrow_with(&mut font_system);
    let attrs = Attrs::new();
    buffer.set_text("Hello rust 🦀", &attrs, Shaping::Advanced);
    buffer.shape_until_scroll(true);

    let text_color = cosmic_text::Color::rgb(0, 0, 0);
    for run in buffer.layout_runs() {
        for glyph in run.glyphs {
            dbg!(glyph);
        }
    }
    let mut image = RgbaImage::new(200, 200);
    buffer.draw(
        &mut swash_cache,
        text_color,
        |x, y, width, height, color| {
            let [r, g, b, a] = color.as_rgba();
            image.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a]));
        },
    );
    image.save("render.png").unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_cosmic_text() {
        render_text();
    }
}
