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
use crate::surface::RectSurface;
use crate::widgets::Widget;
use crate::app::view::View;
use crate::app::App;
#[macro_use]
extern crate glium;

fn main() {
	widget_tree()
}


fn widget_tree() {
	let rect = Rect::new(500, 200, Colour::Rgb(24, 24, 24));

	let container = Container::new(rect).padding(12).colour(Colour::Rgb(255, 25, 255));

	let page = View::new(container);

	let app = 
		App::new()
		.add_view(page);

	app.run();

}

