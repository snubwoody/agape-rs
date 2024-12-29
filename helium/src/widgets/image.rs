use crystal::{BoxSizing, EmptyLayout};
use image::{DynamicImage, GenericImageView, ImageReader};
use crate::{surface::image::ImageSurface, widgets::WidgetBody};
use super::Widget;

/// The source of an image passed to the [`Image`] `widget`
pub enum ImageSource {
	File(&'static str),
	Bytes,
	Url
}

// TODO maybe impl into?
// TODO impl into for the image crate?

pub struct Image{
	id:String,
	image:image::DynamicImage,	
	layout:crystal::EmptyLayout
}

impl Image {
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

	pub fn url() -> Self{
		todo!()
	}

	pub fn bytes() -> Self{
		todo!()
	}
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
}
