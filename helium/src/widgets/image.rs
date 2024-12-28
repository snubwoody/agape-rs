use crystal::{BoxSizing, EmptyLayout};

use crate::{surface::image::ImageSurface, widgets::WidgetBody};

use super::Widget;



pub struct Image{
	
}

impl Widget for Image {
	fn build(&self) -> (super::WidgetBody,Box<dyn crystal::Layout>) {
		// TODO should probably find some way to pass the id's automatically
		let id = nanoid::nanoid!();
		let surface = ImageSurface::new("C:/Users/wakun/Projects/Tools/Rust-UI/examples/temp/legends never die.png").unwrap();
		let mut layout = EmptyLayout::new();
		layout.intrinsic_size.width = BoxSizing::Fixed(200.0);
		layout.intrinsic_size.height = BoxSizing::Fixed(200.0);
		layout.id = id.clone();
		
		let body = WidgetBody{
			id,
			surface:Box::new(surface),
			label:Some("Image".to_owned()),
			..Default::default()
		}; 

		(body,Box::new(layout))
	}
}