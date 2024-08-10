use crate::{
	app::events::EventFunction, colour::Colour, layout::Layout, surface::Surface, widgets::WidgetBody
};
use super::Widget;

pub struct Button{
	pub text:String,
	pub colour:Colour,
	pub on_click:Option<EventFunction>,
	pub on_hover:Option<EventFunction>,
}

impl Button {
	pub fn new(text:&str) -> Self {
		Self { 
			text:text.into(), 
			colour:Colour::Rgb(255, 255, 255),
			on_click:None,
			on_hover:None,
		}
	}

	pub fn colour(mut self,colour:Colour) -> Self {
		self.colour = colour;
		self
	}

	pub fn on_hover(mut self,f:impl Fn() + 'static) -> Self{
		self.on_click = Some(EventFunction::OnHover(Box::new(f)));
		self
	}

	pub fn on_click(mut self,f:impl Fn() + 'static) -> Self{
		self.on_click = Some(EventFunction::OnClick(Box::new(f)));
		self
	}
}

impl Widget for Button {
	fn build(self) -> WidgetBody {
		let surface = Surface::new(0, 0, 200, 70, Colour::Rgb(255, 225, 255));
		let layout = Layout::SingleChild { width: 250, height: 70 };
		let mut events = vec![];

		match self.on_click {
			Some(function) => events.push(function),
			None => {}
		}

		match self.on_hover {
			Some(function) => events.push(function),
			None => {}
		}

		WidgetBody { 
			surface,
			layout,
			children:vec![],
			events 
		}

	}
}