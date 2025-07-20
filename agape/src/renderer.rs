use crate::FONT;
use crate::style::Border;
use agape_core::{Color, Position, Rgba, Size, map};
use fontdue::{Font, FontSettings};
use tiny_skia::{IntSize, Paint, PathBuilder, Pixmap, PixmapPaint, Stroke, Transform};

/// Draw a rectangle onto the `Pixmap`.
pub fn draw_rect(
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
pub fn draw_text(pixmap: &mut Pixmap, text: &str, position: Position) {
    let font = FONT.get().unwrap();
    let font_size = 16.0;

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

pub fn init_font() -> Font {
    let bytes = include_bytes!("../fonts/Inter/static/Inter-Regular.ttf") as &[u8];
    Font::from_bytes(bytes, FontSettings::default()).unwrap()
}
