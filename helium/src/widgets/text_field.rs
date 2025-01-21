use std::cell::Cell;
use crystal::EmptyLayout;
use crate::{colors::tailwind_colors::GRAY200, events::{Element, EventContext, EventFn, Key}, view::{RectView, TextView}};
use super::Widget;

/// Contains editable text
pub struct TextField<'a>{
	id:String,
	text:Cell<&'a str>,
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
		}
	}

	pub fn on_click(mut self,f:impl FnMut() + 'static) -> Self{
		self
	}

	pub fn click(&mut self){
		//self.focused = !self.focused;
		//self.cursor.blink();
		//self.border.color = Colors::Blue;
	}

	fn on_input(&mut self){

	}
}

impl<'a> Widget for TextField<'a> {
	fn id(&self) -> &str {
		&self.id
	}

	fn tick(&mut self,elements:&[Element]) {
		
	}

	fn layout(&self) -> Box<dyn crystal::Layout> {
		let mut layout = EmptyLayout::new();
		layout.id = self.id.clone();
		Box::new(layout)
	}

	fn view(&self) -> Box<dyn crate::view::View> {
		Box::new(RectView::new(&self.id).color(GRAY200))
	}
}