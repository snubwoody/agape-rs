use super::{LayoutConfig, LayoutType, Widget, WidgetBody};
use crate::{impl_layout, Result};
use crystal::{BoxSizing, EmptyLayout, Layout};
use helium_renderer::IntoSurface;
use image::{GenericImageView, ImageReader};
use resvg::tiny_skia::Pixmap;

/// Represents the state of the [`Image`]
#[derive(Debug, Clone, PartialEq)]
enum ImageState {
    Loading(String),
    Complete(image::DynamicImage),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    id: String,
    state: ImageState,
    layout: crystal::EmptyLayout,
}

// FIXME return all the errors from the methods
impl Image {
    // TODO add new constructor that takes in an image
    // TODO add bytes() method
    pub fn file(path: &str) -> Self {
        // TODO make this async
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
    pub fn url(url: &str) -> Result<Self> {
        let id = nanoid::nanoid!();

        let img = reqwest::blocking::get(url)?.bytes()?;
        let image = image::load_from_memory(&img)?;

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

    /// Create an [`Image`] from raw bytes
    pub fn bytes(bytes: &[u8]) -> Result<Self> {
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

    impl_layout!();
}

impl Widget for Image {
    fn id(&self) -> &str {
        &self.id
    }

	fn build(&self,_renderer: &mut helium_renderer::Renderer) -> WidgetBody {
		let primitive = match &self.state {
            ImageState::Complete(image) => 
                helium_renderer::ImageSurface::new(image.to_rgba8().clone())
					.into_surface(),
            ImageState::Loading(_) =>
                helium_renderer::RectSurface::default()
					.into_surface()
        };


		let layout = LayoutConfig::new()
			.layout(LayoutType::EmptyLayout);

		WidgetBody { 
			id: self.id.clone(), 
			layout, 
			primitive, 
			children: vec![]
		}
	}

    fn layout(&self, _: &mut helium_renderer::Renderer) -> Box<dyn crystal::Layout> {
        Box::new(self.layout.clone())
    }

    fn draw(&self, layout: &dyn crystal::Layout, renderer: &mut helium_renderer::Renderer) {
        match &self.state {
            ImageState::Complete(image) => {
                renderer.draw([helium_renderer::ImageSurface::new(image.to_rgba8().clone())
                    .position(layout.position().x, layout.position().y)
                    .size(layout.size().width, layout.size().height)]);
            }
            ImageState::Loading(_) => {
                renderer.draw([helium_renderer::RectSurface::default()
                    .position(layout.position().x, layout.position().y)]);
            }
        }
    }
}
