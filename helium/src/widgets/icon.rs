use super::Widget;
use crate::{impl_widget, view::View};
use crystal::{BoxSizing, EmptyLayout};
use helium_core::color::Color;
use image::GenericImageView;
use resvg::tiny_skia::Pixmap;
use std::fs;

/// Contains all the icons from the [feather icons](https://feathericons.com/) library
// TODO add this behind a feature flag since it increased binary size
pub mod feather_icons {
    use helium_macros::include_icons;

    // The path is relative to the root crate
    include_icons!("./helium/icons/feather-icons");
}

pub struct Icon {
    id: String,
    image: image::DynamicImage,
    layout: crystal::EmptyLayout,
    color: Color,
}

impl Icon {
    /// Create an icon by passing in the raw bytes, the bytes must be
    /// from an svg image.
    pub fn bytes(bytes: &[u8]) -> Self {
        let id = nanoid::nanoid!();

        let options = usvg::Options::default();
        let tree = usvg::Tree::from_data(&bytes, &options).unwrap();

        let mut pixmap = Pixmap::new(24, 24).unwrap();

        resvg::render(
            &tree,
            resvg::tiny_skia::Transform::default(),
            &mut pixmap.as_mut(),
        );
        let png_data = pixmap.encode_png().unwrap();

        let image = image::load_from_memory(&png_data).unwrap();

        let mut layout = EmptyLayout::new();
        layout.intrinsic_size.width = BoxSizing::Fixed(image.dimensions().0 as f32);
        layout.intrinsic_size.height = BoxSizing::Fixed(image.dimensions().1 as f32);
        layout.id = id.clone();

        Self {
            id,
            image,
            layout,
            color: Color::Rgb(0, 0, 0),
        }
    }

    /// Create an icon from a file
    pub fn svg(path: &str) -> Self {
        let id = nanoid::nanoid!();
        let svg_data = fs::read(path).unwrap();

        let options = usvg::Options::default();
        let tree = usvg::Tree::from_data(&svg_data, &options).unwrap();

        let mut pixmap = Pixmap::new(24, 24).unwrap();

        resvg::render(
            &tree,
            resvg::tiny_skia::Transform::default(),
            &mut pixmap.as_mut(),
        );
        let png_data = pixmap.encode_png().unwrap();

        let image = image::load_from_memory(&png_data).unwrap();

        let mut layout = EmptyLayout::new();
        layout.intrinsic_size.width = BoxSizing::Fixed(image.dimensions().0 as f32);
        layout.intrinsic_size.height = BoxSizing::Fixed(image.dimensions().1 as f32);
        layout.id = id.clone();

        Self {
            id,
            image,
            layout,
            color: Color::Rgb(0, 0, 0),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    impl_widget!();
}

impl Widget for Icon {
    fn layout(&self) -> Box<dyn crystal::Layout> {
        Box::new(EmptyLayout::default())
    }

    fn primitive(&self) -> crate::view::Primitive {
        Primitive::Icon {
            id: self.id.clone(),
            image: self.image.clone(),
        }
    }
}
