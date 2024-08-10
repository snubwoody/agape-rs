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
	let button = Button{text:"Click me".to_string()};

	let mut tree = WidgetTree::new();
	tree.build(button);

	let page = View{
		widget_tree:tree
	};

	let app = 
		App::new()
		.add_view(page);

	app.run();
	//dbg!(page);

}

