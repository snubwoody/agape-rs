//! Image support.
//!
//! Only the following image formats are supported:
//! - `PNG`
//! - `WebP`
//! - `JPEG`

use crate::Error::UnsupportedImageFormat;
use crate::impl_style;
use crate::style::BoxStyle;
use crate::widgets::{LayoutDescription, LayoutType, RenderBox, RenderObject, Widget};
use agape_core::{GlobalId, Size};
use agape_layout::{EmptyLayout, IntrinsicSize, Layout};
use agape_renderer::Renderer;
use image::{DynamicImage, ImageFormat, ImageReader};
use std::fs;
use std::io::Cursor;
use std::path::Path;
use tiny_skia::Pixmap;

// TODO: re-export image format
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
    data: DynamicImage,
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
            data: image,
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

    fn layout(&self, _: &mut Renderer) -> Box<dyn Layout> {
        let layout = EmptyLayout {
            id: self.id,
            intrinsic_size: self.style.intrinsic_size.clone(),
            ..Default::default()
        };

        Box::new(layout)
    }
    fn render(&self, pixmap: &mut Pixmap, renderer: &mut Renderer, layout: Box<dyn Layout>) {
        let layout = layout.get(self.id()).unwrap();
        let size = layout.size();
        let position = layout.position();
        renderer.draw_image(pixmap, &self.data, position, size);
    }

    fn build(&self, _: &mut Renderer) -> RenderBox {
        let layout_desc = LayoutDescription {
            intrinsic_size: self.style.intrinsic_size,
            layout_type: LayoutType::EmptyLayout,
            ..Default::default()
        };

        let render_object = RenderObject::Image {
            image: self.data.clone(),
        };

        RenderBox::new(self.id, layout_desc, render_object)
    }
}
