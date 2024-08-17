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
use widgets::text::Text;
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
	let text = Text::new("Hello worlf");

	let page = View::new(text);

	let app = 
		App::new()
		.add_view(page);

	app.run();

}

