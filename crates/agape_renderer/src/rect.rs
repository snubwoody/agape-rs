use agape_core::{Border, Color, IntoColor, Position, Rgba, Size, map};
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};

// TODO: add builder
#[derive(Debug, Clone, PartialOrd, PartialEq, Default)]
pub struct Rect {
    pub size: Size,
    pub position: Position,
    pub color: Color<Rgba>,
    pub corner_radius: u32,
    pub border: Option<Border>,
}

impl Rect {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size.width = width;
        self.size.height = height;
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position.x = x;
        self.position.y = y;
        self
    }

    pub fn color(mut self, color: impl IntoColor<Rgba>) -> Self {
        self.color = color.into_color();
        self
    }

    pub fn border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    pub fn corner_radius(mut self, radius: u32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub(crate) fn draw(&self, pixmap: &mut Pixmap) {
        if self.size.width == 0.0 || self.size.height == 0.0 {
            return;
        }
        let (r, g, b, a) = self.color.inner();

        // Map the alpha since it's clipped to 100
        // TODO: change alpha to f32
        let a = map(a as f32, [0.0, 100.0], [0.0, 255.0]) as u8;
        let mut paint = Paint::default();
        paint.set_color_rgba8(r, g, b, a);

        let Position { x, y } = self.position;
        let Size { width, height } = self.size;
        // TODO: get max radius
        let radius = self.corner_radius as f32;

        // Construct a rounded rect going clockwise
        let mut pb = PathBuilder::new();
        pb.move_to(x + radius, y);
        // Top edge
        pb.line_to(x + width - radius, y);
        // Top right corner
        pb.quad_to(x + width, y, x + width, y + radius);
        // Right edge
        pb.line_to(x + width, y + height - radius);
        // Bottom right corner
        pb.quad_to(x + width, y + height, x + width - radius, y + height);
        // Bottom edge
        pb.line_to(x + radius, y + height);
        // Bottom left corner
        pb.quad_to(x, y + height, x, y + height - radius);
        // Left edge
        pb.line_to(x, y + radius);
        // Top left corner
        pb.quad_to(x, y, x + radius, y);

        let path = pb.finish().unwrap();
        let rect = tiny_skia::Rect::from_xywh(x, y, width, height).unwrap();
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );

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

    #[test]
    fn size() {
        let mut pixmap = Pixmap::new(100, 100).unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);
        let rect = Rect::new().size(50.0, 20.0).color((12, 144, 240));
        rect.draw(&mut pixmap);
        for x in 0..100 {
            for y in 0..100 {
                let pixel = pixmap.pixel(x, y).unwrap();
                let r = pixel.red();
                let g = pixel.green();
                let b = pixel.blue();

                if x < 50 && y < 20 {
                    assert_eq!((r, g, b), (12, 144, 240));
                } else {
                    assert_eq!((r, g, b), (255, 255, 255));
                }
            }
        }
    }

    #[test]
    fn background_color() {
        let mut pixmap = Pixmap::new(100, 100).unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);
        let rect = Rect::new().size(100.0, 100.0).color(150);
        rect.draw(&mut pixmap);
        for pixel in pixmap.pixels() {
            assert_eq!(pixel.red(), 150);
            assert_eq!(pixel.green(), 150);
            assert_eq!(pixel.blue(), 150);
        }
    }
}
