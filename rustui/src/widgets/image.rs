use crate::{
	layout::Layout, 
	surface::{image::ImageSurface, Surface}, 
	widgets::{Widget, WidgetBody}
};


// TODO add width and height
/// Simple image widget
pub struct Image{
	pub path:String,
	pub width:u32,
	pub height:u32
}

impl Image {
	pub fn new(path:&str,width:u32,height:u32) -> Self{
		Self { path:path.to_owned(), width, height }
	}
}

impl Widget for Image {
	fn build(&self) -> WidgetBody {
		let surface = Box::new(ImageSurface::new(&self.path,self.width,self.height));
		let size = surface.get_size();
		let layout = Layout::SingleChild { width: size.0, height:size.1 };

		WidgetBody{
			surface,
			layout,
			..Default::default()
		}
	}
}