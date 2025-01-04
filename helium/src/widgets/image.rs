use std::{fs, thread};
use crystal::{BoxSizing, EmptyLayout};
use image::{GenericImageView, ImageReader};
use resvg::tiny_skia::Pixmap;
use crate::{impl_widget, surface::image::ImageSurface, widgets::WidgetBody};
use super::Widget;

pub struct Image{
	id:String,
	image:image::DynamicImage,	
	layout:crystal::EmptyLayout
}

impl Image {
	/// 
	pub fn file(path:&str) -> Self{
		let id = nanoid::nanoid!();
		// TODO handle the error
		let image = ImageReader::open(path).unwrap().decode().unwrap();
		
		let mut layout = EmptyLayout::new();
		layout.intrinsic_size.width = BoxSizing::Fixed(image.dimensions().0 as f32);
		layout.intrinsic_size.height = BoxSizing::Fixed(image.dimensions().1 as f32);
		layout.id = id.clone();

		Self { 
			id,
			image, 
			layout 
		}
		
	}

	/// Create an svg image by passing the raw bytes
	pub fn svg_bytes(bytes:&[u8]) -> Self{
		let id = nanoid::nanoid!();

		let options = usvg::Options::default();
		let tree = usvg::Tree::from_data(&bytes, &options).unwrap();

		let mut pixmap = Pixmap::new(24, 24).unwrap();

		resvg::render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());
		let png_data = pixmap.encode_png().unwrap();

		let image = image::load_from_memory(&png_data).unwrap();
		
		let mut layout = EmptyLayout::new();
		layout.intrinsic_size.width = BoxSizing::Fixed(image.dimensions().0 as f32);
		layout.intrinsic_size.height = BoxSizing::Fixed(image.dimensions().1 as f32);
		layout.id = id.clone();

		Self { 
			id,
			image, 
			layout 
		}
	}

	/// Create an image from an svg, the svg is parsed and rendered into a png.
	pub fn svg(path:&str) -> Self{
		let id = nanoid::nanoid!();
		let svg_data = fs::read(path).unwrap();

		let options = usvg::Options::default();
		let tree = usvg::Tree::from_data(&svg_data, &options).unwrap();

		let mut pixmap = Pixmap::new(24, 24).unwrap();

		resvg::render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());
		let png_data = pixmap.encode_png().unwrap();

		let image = image::load_from_memory(&png_data).unwrap();
		
		let mut layout = EmptyLayout::new();
		layout.intrinsic_size.width = BoxSizing::Fixed(image.dimensions().0 as f32);
		layout.intrinsic_size.height = BoxSizing::Fixed(image.dimensions().1 as f32);
		layout.id = id.clone();

		Self { 
			id,
			image, 
			layout 
		}
	}

	/// Create an image from a url
	pub fn url(url:&str) -> Self{
		let id = nanoid::nanoid!();
		let img = reqwest::blocking::get(url).unwrap().bytes().unwrap();

		let image = image::load_from_memory(&img).unwrap();
		
		let mut layout = EmptyLayout::new();
		layout.intrinsic_size.width = BoxSizing::Fixed(image.dimensions().0 as f32);
		layout.intrinsic_size.height = BoxSizing::Fixed(image.dimensions().1 as f32);
		layout.id = id.clone();
		
		Self{
			id,
			image,
			layout
		}
	}

	pub fn bytes() -> Self{
		todo!()
	}

	impl_widget!();
}

impl Widget for Image {
	fn build(&self) -> (super::WidgetBody,Box<dyn crystal::Layout>) {
		let surface = ImageSurface::new(self.image.clone());

		let body = WidgetBody{
			id:self.id.clone(),
			surface:Box::new(surface),
			label:Some("Image".to_owned()),
			..Default::default()
		}; 

		(body,Box::new(self.layout.clone()))
	}

	fn update(&mut self) {
		let _ = thread::spawn(||{
			println!("Hello");
		}).join();
	}
}
