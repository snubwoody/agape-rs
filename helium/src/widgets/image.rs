use super::{icon::feather_icons, Widget};
use crate::{impl_widget, surface::{image::ImageSurface, rect::RectSurface}, widgets::WidgetBody};
use crystal::{BoxSizing, EmptyLayout};
use helium_core::color::WHITE;
use image::{GenericImageView, ImageReader};
use resvg::tiny_skia::Pixmap;
use std::{fs, thread};

/// Represents the state of the [`Image`]
enum ImageState{
	Loading(String),
	Complete(image::DynamicImage)
}

pub struct Image {
    id: String,
    state:ImageState,
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
			state:ImageState::Complete(image), 
			layout 
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
			state:ImageState::Complete(image), 
			layout 
		}
    }

    /// Create an image from an svg, the svg is parsed and rendered into a png.
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
			state:ImageState::Complete(image), 
			layout 
		}
    }

    /// Create an image from a url
    pub fn url(url: &str) -> Self {
        let id = nanoid::nanoid!();

        let mut layout = EmptyLayout::new();
        layout.id = id.clone();

        Self { 
			id, 
			state:ImageState::Loading(url.to_string()), 
			layout 
		}
    }

    pub fn bytes() -> Self {
        todo!()
    }

    impl_widget!();
}

impl Widget for Image {
    fn build(&self) -> (WidgetBody, Box<dyn crystal::Layout>) {

        let body = match &self.state {
			ImageState::Complete(image) => {
				WidgetBody {
					id: self.id.clone(),
					surface: Box::new(ImageSurface::new(image.clone())),
					label: Some("Image".to_owned()),
					..Default::default()
				}
			},
			ImageState::Loading(_) => {
				WidgetBody {
					id: self.id.clone(),
					surface:Box::new(RectSurface::new(0.0, 0.0, 0.0, 0.0, WHITE)),
					label: Some("Image".to_owned()),
					..Default::default()
				}
			}
		};

        

        (body, Box::new(self.layout.clone()))
    }

    fn update(&mut self) {
		let img;
		if let ImageState::Loading(url) = &self.state{
			img = reqwest::blocking::get(url).unwrap().bytes().unwrap();
		}else {
			return;
		}

        let image = image::load_from_memory(&img).unwrap();
		
        self.layout.intrinsic_size.width = BoxSizing::Fixed(image.dimensions().0 as f32);
        self.layout.intrinsic_size.height = BoxSizing::Fixed(image.dimensions().1 as f32);
        
		self.state = ImageState::Complete(image);
    }
}
