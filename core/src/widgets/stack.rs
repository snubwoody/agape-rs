use crate::{
	layout::Layout, 
	widgets::{Widget, WidgetBody}
};


// The stacks aren't working, every change breaks the 
// stacks is some way, it's actually so infuriating, i
// don't even know what to do about them.
pub struct VStack{
	pub spacing:u32,
	pub padding:u32,
	pub children:Vec<Box<dyn Widget>>
}

impl Widget for VStack {
	fn build(self) -> WidgetBody {
		let layout = Layout::Vertical { spacing: self.spacing, padding: self.padding };
		let children:Vec<Box<WidgetBody>> = self.children.iter().map(|widget| {
			Box::new(widget.build())
		}).collect();

		WidgetBody{
			layout,
			children,
			..Default::default()
		}
	}
}

pub struct HStack{
	pub spacing:u32,
	pub padding:u32,
	pub children:Vec<Box<dyn Widget>>
}

impl Widget for HStack {
	fn build(self) -> WidgetBody {
		let layout = Layout::Horizontal  { spacing: self.spacing, padding: self.padding };
		
		let children:Vec<Box<WidgetBody>> = self.children.iter().map(|widget|{
			Box::new(widget.build())
		}).collect();

		WidgetBody{
			layout,
			children,
			..Default::default()
		}

	}
}

