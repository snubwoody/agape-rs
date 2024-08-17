mod widgets;
mod colour;
mod app;
pub mod utils;
pub mod surface;
pub mod text;
pub mod vertex;
pub mod layout;
use colour::Colour;
use widgets::list::TextList;
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
	let text = Text::new("Three frogs flew through france");
	let rect = Rect::new(500, 50, Colour::Rgb(35,35, 35));
	let text_list = TextList::new(vec![
		"First item",
		"Second item"
	]);

	let vstack = VStack{
		spacing:12,
		padding:12,
		children:vec![
			Box::new(text),
			Box::new(rect),
			Box::new(text_list),
		]
	};

	let page = View::new(vstack);

	let app = 
		App::new()
		.add_view(page);

	app.run();
}

