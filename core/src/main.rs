mod widgets;
mod colour;
mod app;
pub mod utils;
pub mod surface;
pub mod text;
pub mod vertex;
pub mod layout;
use colour::Colour;
use widgets::button::Button;
use widgets::container::Container;
use widgets::rect::Rect;
use widgets::stack::{HStack, VStack};
use widgets::WidgetTree;
use crate::surface::Surface;
use crate::widgets::Widget;
use crate::app::view::View;
use crate::app::App;
#[macro_use]
extern crate glium;

fn main() {
	widget_tree()
}


fn widget_tree() {
	let button = Button::new("Hello").on_hover(||{dbg!("I was hovered over");});

	let page = View::new(button);

	let app = 
		App::new()
		.add_view(page);

	app.run();

}

