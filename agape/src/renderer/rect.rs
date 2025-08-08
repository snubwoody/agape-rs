use agape_core::{Border, Color, Position, Rgba, Size, map};
use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke, Transform};

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
