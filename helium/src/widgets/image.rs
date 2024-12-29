use crystal::{BoxSizing, EmptyLayout};
use image::ImageReader;
use crate::{surface::image::ImageSurface, widgets::WidgetBody};
use super::Widget;

// TODO create an image macro similar to include bytes
pub struct Image{
	
}

impl Widget for Image {
	fn build(&self) -> (super::WidgetBody,Box<dyn crystal::Layout>) {
		// TODO should probably find some way to pass the id's automatically
		let id = nanoid::nanoid!();
		let path = "C:/Users/wakun/Projects/Tools/Rust-UI/examples/temp/legends never die.png";
		let path = "c:/Users/wakun/Projects/Tools/Rust-UI/examples/temp/image.jpg";

		let img = ImageReader::open(path).unwrap().decode().unwrap().to_rgba8();
		
		let mut layout = EmptyLayout::new();
		layout.intrinsic_size.width = BoxSizing::Fixed(img.dimensions().0 as f32);
		layout.intrinsic_size.height = BoxSizing::Fixed(img.dimensions().1 as f32);
		layout.id = id.clone();
		
		let surface = ImageSurface::new(img);

		let body = WidgetBody{
			id,
			surface:Box::new(surface),
			label:Some("Image".to_owned()),
			..Default::default()
		}; 

		(body,Box::new(layout))
	}
}
