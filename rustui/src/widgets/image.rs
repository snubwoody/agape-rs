use crate::{layout::Layout, surface::{image::ImageSurface, Surface}, widgets::{Widget, WidgetBody}};


pub struct Image{
	pub path:String
}

impl Image {
	pub fn new(){
		
	}
}

impl Widget for Image {
	fn build(&self) -> WidgetBody {
		let surface = Box::new(ImageSurface::new(&self.path));
		let size = surface.get_size();
		let layout = Layout::SingleChild { width: size.0, height:size.1 };

		WidgetBody{
			surface,
			layout,
			..Default::default()
		}
	}
}