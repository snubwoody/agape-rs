use super::Widget;
use crate::impl_widget;
use crystal::{BoxSizing, EmptyLayout};
use image::{GenericImageView, ImageReader};
use resvg::tiny_skia::Pixmap;

/// Represents the state of the [`Image`]
#[derive(Debug, Clone, PartialEq)]
enum ImageState {
    Loading(String),
    Complete(image::DynamicImage),
}

/// An Image.  
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    id: String,
    state: ImageState,
    layout: crystal::EmptyLayout,
}

// FIXME return all the errors from the methods
impl Image {
    pub fn file(path: &str) -> Self {
        let id = nanoid::nanoid!();
        // TODO handle the error
        let image = ImageReader::open(path).unwrap().decode().unwrap();

        let mut layout = EmptyLayout::new();
        layout.intrinsic_size.width = BoxSizing::Fixed(image.dimensions().0 as f32);
        layout.intrinsic_size.height = BoxSizing::Fixed(image.dimensions().1 as f32);
        layout.id = id.clone();

        Self {
            id,
            state: ImageState::Complete(image),
            layout,
        }
    }

    /// Create an svg image by passing the raw bytes
    pub fn svg_bytes(bytes: &[u8]) -> Self {
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
            state: ImageState::Complete(image),
            layout,
        }
    }

    /// Create an image from an svg, the svg is parsed and rendered into a png.
    pub fn svg(path: &str) -> Self {
        let id = nanoid::nanoid!();
        let svg_data = std::fs::read(path).unwrap();

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
            state: ImageState::Complete(image),
            layout,
        }
    }

    /// Create an image from a url
    pub fn url_sync(url: &str) -> Self {
        let id = nanoid::nanoid!();

        let img = reqwest::blocking::get(url).unwrap().bytes().unwrap();
        let image = image::load_from_memory(&img).unwrap();

        let mut layout = EmptyLayout::new();
        layout.id = id.clone();

        Self {
            id,
            state: ImageState::Complete(image),
            layout,
        }
    }

    /// Load an image from a url
    ///
    /// # Caution
    ///
    /// This method of creating an image *is* working, however it is currently
    /// blocking which is quite noticeable when loading multiple images. Currently
    /// exploring ways to provide non-blocking ways of loading data.
    pub fn url(url: &str) -> Self {
        let id = nanoid::nanoid!();

        let img = reqwest::blocking::get(url).unwrap().bytes().unwrap();
        let image = image::load_from_memory(&img).unwrap();

        let mut layout = EmptyLayout::new();
        layout.id = id.clone();
        layout.intrinsic_size.width = BoxSizing::Fixed(image.dimensions().0 as f32);
        layout.intrinsic_size.height = BoxSizing::Fixed(image.dimensions().1 as f32);

        Self {
            id,
            state: ImageState::Complete(image),
            layout,
        }
    }

    /// Create an [`Image`] from raw bytes
    pub fn bytes(bytes: &[u8]) -> Result<Self, image::ImageError> {
        let id = nanoid::nanoid!();
        let image = image::load_from_memory(bytes)?;

        let mut layout = EmptyLayout::new();
        layout.id = id.clone();
        layout.intrinsic_size.width = BoxSizing::Fixed(image.dimensions().0 as f32);
        layout.intrinsic_size.height = BoxSizing::Fixed(image.dimensions().1 as f32);

        Ok(Self {
            id,
            state: ImageState::Complete(image),
            layout,
        })
    }

    impl_widget!();
}

impl Widget for Image {
    fn id(&self) -> &str {
        &self.id
    }

    fn tick(&mut self, elements: &[crate::events::Element]) {}

    fn layout(&self) -> Box<dyn crystal::Layout> {
        Box::new(self.layout.clone())
    }

	fn draw(&self,layout:&dyn crystal::Layout,renderer:&mut helium_renderer::Renderer) {
		match &self.state {
            ImageState::Complete(image) => {
				renderer.draw([
					helium_renderer::Image::new(image.clone())
						.position(layout.position().x, layout.position().y)
				]);
			},
            ImageState::Loading(_) => {
                renderer.draw([
					helium_renderer::Rect::default()
						.position(layout.position().x, layout.position().y)
				]);
            }
        }
		
	}
}
