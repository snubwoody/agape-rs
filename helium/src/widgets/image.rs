use crate::{
	layout::Layout, 
	surface::{image::ImageSurface, Surface}, 
	widgets::{Widget, WidgetBody}
};


/// Simple image widget
#[derive(Debug)]
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
		let surface = Box::new(
			ImageSurface::new(&self.path,self.width as f32,self.height as f32)
		);
		let size = surface.get_size();
		let layout = Layout::Block { padding: 0 };

		WidgetBody{
			surface,
			layout,
			..Default::default()
		}
	}

	fn get_children(self:Box<Self>) -> Vec<Box<dyn Widget>> {
		vec![]
	}
}