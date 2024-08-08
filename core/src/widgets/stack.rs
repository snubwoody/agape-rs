use crate::{
	layout::Layout, 
	surface::Surface, 
	widgets::{Widget, WidgetBody}
};

#[derive(Debug)]
pub struct VStack{
	pub spacing:u32,
	pub padding:u32,
	pub children:Vec<Box<dyn Widget>>
}

impl Widget for VStack {
	fn build(&self) -> WidgetBody {
		let surface = Surface::default();
		let layout = Layout::Vertical { spacing: self.spacing, padding: self.padding };
		let children = self.children.iter().map(|widget| Box::new(widget.build())).collect();

		WidgetBody{
			surface,
			layout,
			children
		}

	}
}

#[derive(Debug)]
pub struct HStack{
	pub spacing:u32,
	pub padding:u32,
	pub children:Vec<Box<dyn Widget>>
}

impl Widget for HStack {
	fn build(&self) -> WidgetBody {
		let surface = Surface::default();
		let layout = Layout::Horizontal  { spacing: self.spacing, padding: self.padding };
		let children = self.children.iter().map(|widget| Box::new(widget.build())).collect();

		WidgetBody{
			surface,
			layout,
			children
		}

	}
}

