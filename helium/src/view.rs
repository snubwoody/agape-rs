//! [`View`]'s are responsible for drawing widgets to the screen, as such 
//! they hold rendering information such as size, position and color. 
//! Rendering is done using `tiny_skia`.
//!
//! There is one kind of view:
//! - [`RectView`]

use helium_core::{Color, GlobalId, IntoColor, Position, Rgba, Size, map};
use tiny_skia::{Paint, Pixmap, Transform};

/// A [`View`] is a primitive object that performs the rendering to the screen.
pub trait View {
    /// Get the view's id
    fn id(&self) -> GlobalId;
    /// Get the view's color
    fn color(&self) -> &Color<Rgba>;
    
    fn set_id(&mut self, id: GlobalId);
    fn set_size(&mut self, size: Size);
    fn set_position(&mut self, position: Position);
    
    /// Render the view to the screen. 
    fn render(&self, pixmap: &mut Pixmap);
}

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
        pixmap.fill(tiny_skia::Color::WHITE);
        let (r, g, b, a) = self.color.inner();

        // Map the color since we clip the alpha to 100
        let a = map(a as f32, [0.0, 100.0], [0.0, 255.0]);
        let mut paint = Paint::default();
        paint.set_color_rgba8(r, g, b, a as u8);

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
