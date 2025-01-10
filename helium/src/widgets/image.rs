use super::Widget;
use crate::{
    impl_widget,
	surface::Primitive
};
use crystal::{BoxSizing, EmptyLayout};
use helium_core::color::BLACK;
use image::{GenericImageView, ImageReader};
use resvg::tiny_skia::Pixmap;

/// Represents the state of the [`Image`]
#[derive(Debug)]
enum ImageState {
    Loading(String),
    Complete(image::DynamicImage),
}

pub struct Image {
    id: String,
    state: ImageState,
    layout: crystal::EmptyLayout,
}

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

    pub fn url(url: &str) -> Self {
        let id = nanoid::nanoid!();

        let mut layout = EmptyLayout::new();
        layout.id = id.clone();

        let img = reqwest::blocking::get(url).unwrap().bytes().unwrap();
        let image = image::load_from_memory(&img).unwrap();

        Self {
            id,
            state: ImageState::Complete(image),
            layout,
        }
    }

    pub fn bytes() -> Self {
        todo!()
    }

    impl_widget!();
}

impl Widget for Image {

	fn layout(&self) -> Box<dyn crystal::Layout> {
		Box::new(self.layout.clone())
	}

	fn primitive(&self) -> Primitive {
		match &self.state {
            ImageState::Complete(image) => 
				Primitive::Image { id: self.id.clone(), image: image.clone()},
            ImageState::Loading(_) => 
				Primitive::Rect { id: self.id.clone(), corner_radius: 12, color: BLACK},
        }
	}

    fn update(&mut self) {
        if let ImageState::Loading(url) = &self.state {
            let img = reqwest::blocking::get(url).unwrap().bytes().unwrap();
            let image = image::load_from_memory(&img).unwrap();
            self.state = ImageState::Complete(image);
        }
    }
}
