use super::View;
use crate::Resources;
use crate::style::Border;
use agape_core::{Color, GlobalId, Position, Rgba, Size, map};
use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke, Transform};

/// Responsible for drawing rectangular shapes to the screen.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RectView {
    pub id: GlobalId,
    pub position: Position,
    pub size: Size,
    pub color: Color<Rgba>,
    pub border: Option<Border>,
}

impl RectView {
    /// Create a new rect view.
    pub fn new(id: GlobalId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl View for RectView {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn color(&self) -> &Color<Rgba> {
        &self.color
    }

    fn set_id(&mut self, id: GlobalId) {
        self.id = id
    }

    fn set_position(&mut self, position: Position) {
        self.position = position
    }

    fn set_size(&mut self, size: Size) {
        self.size = size
    }

    fn render(&self, pixmap: &mut Pixmap, _: &Resources) {
        let (r, g, b, a) = self.color.inner();

        // Map the alpha since it's clipped to 100
        let a = map(a as f32, [0.0, 100.0], [0.0, 255.0]) as u8;
        let mut paint = Paint::default();
        paint.set_color_rgba8(r, g, b, a);

        let Position { x, y } = self.position;
        let Size { width, height } = self.size;

        let rect = tiny_skia::Rect::from_xywh(x, y, width, height).unwrap();
        pixmap.fill_rect(rect, &paint, Transform::identity(), None);

        if let Some(border) = &self.border {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Resources;
    use agape_core::Color;

    #[test]
    fn use_correct_position() {
        let mut pixmap = Pixmap::new(500, 500).unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);
        let color = Color::BLACK;
        let mut rect_view = RectView {
            color,
            ..Default::default()
        };
        rect_view.position = Position::new(50.0, 100.0);
        rect_view.size = Size::unit(500.0);
        rect_view.render(&mut pixmap, &Resources::new());

        let x_pos = 50;
        let y_pos = 100;
        for x in 0..x_pos {
            for y in 0..y_pos {
                let pixel = pixmap.pixel(x, y).unwrap();
                let r = pixel.red();
                let b = pixel.blue();
                let g = pixel.green();

                assert_eq!(r, 255);
                assert_eq!(g, 255);
                assert_eq!(b, 255);
            }
        }
    }
}
