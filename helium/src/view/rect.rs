use super::View;
use helium_core::{Color, GlobalId, IntoColor, Position, Rgba, Size, map};
use tiny_skia::{BlendMode, Paint, Pixmap, Transform};

#[derive(Default)]
pub struct RectView {
    id: GlobalId,
    position: Position,
    size: Size,
    color: Color<Rgba>,
}

impl RectView {
    pub fn new(color: impl IntoColor<Rgba>) -> Self {
        let color = color.into_color();

        Self {
            id: GlobalId::new(),
            position: Position::default(),
            size: Size::default(),
            color,
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

    fn render(&self, pixmap: &mut Pixmap) {
        let (r, g, b, a) = self.color.inner();

        // Map the alpha since it's clipped to 100
        let a = map(a as f32, [0.0, 100.0], [0.0, 255.0]) as u8;
        let mut paint = Paint::default();
        paint.blend_mode = BlendMode::SourceOver;
        paint.set_color_rgba8(r, g, b, a);

        let Position { x, y } = self.position;
        let Size { width, height } = self.size;

        let rect = tiny_skia::Rect::from_xywh(x, y, width, height).unwrap();
        pixmap.fill_rect(rect, &paint, Transform::identity(), None);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use helium_core::colors::BLACK;

    #[test]
    fn set_correct_pixel_color() {
        let mut pixmap = Pixmap::new(500, 500).unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);
        let color = Color::rgb(25, 120, 97);
        let mut rect_view = RectView::new(color);
        rect_view.size = Size::unit(500.0);
        rect_view.render(&mut pixmap);

        for pixel in pixmap.pixels() {
            let r = pixel.red();
            let g = pixel.green();
            let b = pixel.blue();
            assert_eq!(r, 25);
            assert_eq!(g, 120);
            assert_eq!(b, 97);

            if !pixel.is_opaque() {
                panic!("Incorrect pixel alpha");
            }
        }
    }

    #[test]
    fn use_correct_position() {
        let mut pixmap = Pixmap::new(500, 500).unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);
        let color = BLACK;
        let mut rect_view = RectView::new(color);
        rect_view.position = Position::new(50.0, 100.0);
        rect_view.size = Size::unit(500.0);
        rect_view.render(&mut pixmap);

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
