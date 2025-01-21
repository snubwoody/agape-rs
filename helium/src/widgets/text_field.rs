use std::cell::Cell;
use crystal::EmptyLayout;
use crate::view::TextView;
use super::Widget;

/// Contains editable text
pub struct TextField<'a>{
	id:String,
	text:Cell<&'a str>,
	events:Vec<Box<dyn FnMut()>>
}

impl<'a> TextField<'a> {
	pub fn new() -> Self{
		let text = Cell::new("");
		let mut events = vec![];

		let on_input = ||{};

		events.push(on_input);
		
		Self{
			id:nanoid::nanoid!(),
			text,
			events:vec![]
		}
	}

	pub fn on_click(&mut self){
		//self.focused = !self.focused;
		//self.cursor.blink();
		//self.border.color = Colors::Blue;
	}
}

impl<'a> Widget for TextField<'a> {
	fn id(&self) -> &str {
		&self.id
	}

	fn layout(&self) -> Box<dyn crystal::Layout> {
		let mut layout = EmptyLayout::new();
		layout.id = self.id.clone();
		Box::new(layout)
	}

	fn view(&self) -> Box<dyn crate::view::View> {
		Box::new(TextView::new(&self.id, self.text.get()))
	}
}