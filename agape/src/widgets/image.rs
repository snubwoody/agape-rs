//! Images.
//!
//! Currently only the following image formats are supported:
//! - `PNG`
//! - `WebP`
//! - `JPEG`

use crate::Error::UnsupportedImageFormat;
use crate::impl_style;
use crate::style::BoxStyle;
use crate::widgets::Widget;
use agape_core::{GlobalId, Size};
use agape_layout::{EmptyLayout, IntrinsicSize, Layout};
use agape_renderer::Renderer;
use image::{DynamicImage, ImageFormat, ImageReader};
use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::sync::Arc;

/// Displays an image to the screen. Only `JPEG`, `PNG` and `WebP` formats
/// are currently supported.
///
/// # Open an image file
/// ```no_run
/// use agape::widgets::Image;
///
/// let image = Image::open("assets/standing-sprite.jpeg");
/// ```
pub struct Image {
    id: GlobalId,
    data: Arc<DynamicImage>,
    style: BoxStyle,
}

impl Image {
    /// Open an image.
    pub fn open<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let data = fs::read(path)?;
        Self::bytes(&data)
    }

    /// Load an image from in-memory bytes.
    pub fn bytes(bytes: &[u8]) -> crate::Result<Self> {
        let buf = Cursor::new(bytes);
        let reader = ImageReader::new(buf).with_guessed_format()?;

        if !matches!(
            reader.format(),
            Some(ImageFormat::Png | ImageFormat::Jpeg | ImageFormat::WebP)
        ) {
            return Err(UnsupportedImageFormat);
        }

        let image = reader.decode()?;
        let width = image.width() as f32;
        let height = image.height() as f32;

        let mut style = BoxStyle::new();
        style.intrinsic_size = IntrinsicSize::fixed(width, height);

        Ok(Self {
            id: GlobalId::new(),
            data: Arc::new(image),
            style,
        })
    }

    /// Get the dimensions of the inner image.
    pub fn dimensions(&self) -> Size {
        Size::new(self.data.width() as f32, self.data.height() as f32)
    }

    impl_style! {}
}

impl Widget for Image {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn traverse(&mut self, _: &mut dyn FnMut(&mut dyn Widget)) {}
    fn layout(&self, _: &mut Renderer) -> Box<dyn Layout> {
        let layout = EmptyLayout {
            id: self.id,
            intrinsic_size: self.style.intrinsic_size,
            ..Default::default()
        };

        Box::new(layout)
    }

    fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
        let layout = layout.get(self.id()).unwrap();
        let size = layout.size();
        let position = layout.position();
        let mut image = agape_renderer::image::Image::new(self.data.clone());
        image.size = size;
        image.position = position;
        renderer.draw_image(image);
    }
}
