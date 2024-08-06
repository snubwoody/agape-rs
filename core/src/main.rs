mod widgets;
mod colour;
mod app;
pub mod utils;
pub mod surface;
pub mod text;
pub mod vertex;
pub mod layout;
use app::view::WidgetTree;
use colour::Colour;
use widgets::container::Container;
use widgets::rect::Rect;
use widgets::stack::{HStack, VStack};
use widgets::text::Text;
use crate::surface::Surface;
use crate::widgets::Widget;
use crate::app::view::View;
use crate::app::App;
#[macro_use]
extern crate glium;

fn main() {
	//run_app();
	widget_tree()
}

fn run_app() {
	let rect = Rect::new(50, 150, 400, 150,Colour::Rgb(0, 20, 200));
	let rect2 = Rect::new(50, 150, 400, 150,Colour::Rgb(0, 20, 200));
	let rect3 = Rect::new(50, 150, 400, 150,Colour::Rgb(0, 20, 200));

	let container = Container::new(rect);
	let page = View::new(container);

	dbg!(&page);

	let app = 
		App::new()
		.add_view(page);

	app.run()
}

fn widget_tree() {
	let rect = Rect::new(50, 150, 400, 150,Colour::Rgb(0, 200, 200));
	let rect1 = Rect::new(50, 150, 400, 150,Colour::Rgb(0, 20, 200));
	let rect2 = Rect::new(50, 150, 400, 150,Colour::Rgb(200, 20, 200));
	let container = Container::new(rect2);
	let tree = WidgetTree::new().build(container);
	//dbg!();

}

