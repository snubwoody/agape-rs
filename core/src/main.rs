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
	//run_app();
	widget_tree()
}

/* fn run_app() {
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
} */

fn widget_tree() {
	let rect = Rect{
		width:200,
		height:200,
		colour:Colour::Rgb(255, 25, 25)
	};

	let rect2 = Rect{
		width:200,
		height:200,
		colour:Colour::Rgb(25, 125, 225)
	};

	let container = Container{
		padding:20,
		colour:Colour::Rgb(155,105, 25),
		child:rect
	};

	let hstack = HStack{
		spacing:24,
		padding:12,
		children:vec![
			Box::new(container),
			Box::new(rect2),
		]
	};

	let mut tree = WidgetTree::new();
	tree.build(hstack);

	let page = View{
		widget_tree:tree
	};

	let app = 
		App::new()
		.add_view(page);

	app.run();
	//dbg!(page);

}

