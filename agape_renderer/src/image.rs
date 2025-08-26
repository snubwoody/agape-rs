use agape_core::{Position, Size};
use image::{DynamicImage, GenericImageView};
use std::rc::Rc;
use std::sync::Arc;
use tiny_skia::{IntSize, Pixmap, PixmapPaint, Transform};

#[derive(PartialEq)]
pub struct Image {
    pub size: Size,
    pub position: Position,
    data: Arc<DynamicImage>,
}

impl Image {
    pub fn new(image: Arc<DynamicImage>) -> Self {
        let size = Size::from(image.dimensions());
        Self {
            data: image,
            size,
            position: Position::default(),
        }
    }

    pub fn draw(&self, pixmap: &mut Pixmap) {
        let (width, height) = self.data.dimensions();
        let data = self.data.to_rgba8().into_raw();
        let pixmap_size = IntSize::from_wh(width, height).unwrap();
        let image_pixmap = Pixmap::from_vec(data, pixmap_size).unwrap();

        let scale_x = self.size.width / image_pixmap.width() as f32;
        let scale_y = self.size.height / image_pixmap.height() as f32;
        let transform = Transform::from_translate(self.position.x, self.position.y)
            .post_scale(scale_x, scale_y);

        let x = self.position.x as i32;
        let y = self.position.y as i32;
        let paint = PixmapPaint::default();
        pixmap.draw_pixmap(x, y, image_pixmap.as_ref(), &paint, transform, None);
    }
}
