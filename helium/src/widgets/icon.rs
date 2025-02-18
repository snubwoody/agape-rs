use super::Widget;
use crate::impl_layout;
use crystal::{BoxSizing, EmptyLayout, Layout};
use helium_core::{Color, IntoColor, Rgba};
use helium_renderer::IntoPrimitive;
use image::GenericImageView;
use resvg::tiny_skia::Pixmap;
use std::fs;

/// Contains all the icons from the [feather icons](https://feathericons.com/) library
// TODO add this behind a feature flag since it increased binary size
#[cfg(feature = "feather-icons")]
pub mod feather_icons {
	// TODO move this to icons module
    use helium_macros::include_icons;

    // The path is relative to the root crate
    include_icons!("./helium/icons/feather-icons");
}

pub struct Icon {
    id: String,
    image: image::DynamicImage,
    layout: crystal::EmptyLayout,
    color: Color<Rgba>,
}

impl Icon {
    /// Create an icon by passing in the raw bytes, the bytes must be
    /// from an svg image.
    pub fn bytes(bytes: &[u8]) -> Self {
        // TODO return a result
        let id = nanoid::nanoid!();

        let options = usvg::Options::default();
        let tree = usvg::Tree::from_data(bytes, &options).unwrap();

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
            color: Color::rgb(0, 0, 0),
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
            color: Color::rgb(0, 0, 0),
        }
    }

    pub fn color(mut self, color: impl IntoColor<Rgba>) -> Self {
        self.color = color.into_color();
        self
    }

    impl_layout!();
}

impl Widget for Icon {
    fn id(&self) -> &str {
        &self.id
    }

	fn build(&self,renderer: &mut helium_renderer::Renderer) -> (Box<dyn crystal::Layout>,helium_renderer::Primitive) {
		let layout = self.layout.clone();

		let primitive = helium_renderer::Icon::new(self.image.clone())
            .color(self.color.clone())
            .position(layout.position().x, layout.position().y)
			.into_primitive();

		(Box::new(layout),primitive)
	}

    fn layout(&self, _: &mut helium_renderer::Renderer) -> Box<dyn crystal::Layout> {
        Box::new(self.layout.clone())
    }

    fn draw(&self, layout: &dyn crystal::Layout, renderer: &mut helium_renderer::Renderer) {
        renderer.draw([helium_renderer::Icon::new(self.image.clone())
            .color(self.color.clone())
            .position(layout.position().x, layout.position().y)]);
    }
}
