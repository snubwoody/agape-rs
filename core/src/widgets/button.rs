use crate::{
	app::events::EventFunction, colour::Colour, layout::Layout, surface::RectSurface, widgets::WidgetBody
};
use super::Widget;

pub struct Button{
	pub text:String,
	pub colour:Colour,
	events:Vec<EventFunction>
}

impl Button {
	pub fn new(text:&str) -> Self {
		Self { 
			text:text.into(), 
			colour:Colour::Rgb(255, 255, 255),
			events:Vec::new()
		}
	}

	pub fn colour(mut self,colour:Colour) -> Self {
		self.colour = colour;
		self
	}

	pub fn on_hover(mut self,f:impl Fn() + 'static) -> Self {
		self.events.push(EventFunction::OnHover(Box::new(f)));
		self
	}

	pub fn on_click(mut self,f:impl Fn() + 'static) -> Self{
		self.events.push(EventFunction::OnClick(Box::new(f)));
		self
	}
}

impl Widget for Button {
	fn build(&self) -> WidgetBody {
		let surface = Box::new(
			RectSurface::new(0.0, 0.0, 200, 70, Colour::Rgb(255, 255, 255))
		);

		let layout = Layout::SingleChild { width: 250, height: 70 };
		// FIXME

		WidgetBody { 
			surface,
			layout,
			..Default::default()
		}
	}
}