use crate::Error::UnsupportedImageFormat;
use crate::impl_style;
use crate::style::BoxStyle;
use crate::widgets::{LayoutDescription, LayoutType, RenderBox, RenderObject, Widget};
use agape_core::{GlobalId, Size};
use agape_layout::IntrinsicSize;
use image::{DynamicImage, ImageFormat, ImageReader};
use std::path::Path;

// TODO: re-export image format
pub struct Image {
    id: GlobalId,
    data: DynamicImage,
    style: BoxStyle,
}

impl Image {
    pub fn open<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let reader = ImageReader::open(path)?.with_guessed_format()?;
        if let Some(format) = reader.format() {
            match format {
                ImageFormat::Png | ImageFormat::Jpeg | ImageFormat::WebP => (),
                _ => return Err(UnsupportedImageFormat),
            }
        } else {
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

    fn build(&self) -> RenderBox {
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

#[cfg(test)]
mod test {
    #[test]
    fn initial_size() {
        todo!("Implement from memory and test inital size")
    }
}
