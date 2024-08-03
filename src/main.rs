mod widgets;
mod view;
mod colour;
mod app;
pub mod utils;
pub mod surface;
pub mod text;
pub mod vertex;
pub mod layout;
use colour::rgb;
use widgets::container::Container;
use widgets::rect::Rect;
use widgets::stack::{HStack, VStack};
use crate::surface::Surface;
use crate::widgets::Widget;
use crate::view::View;
use crate::app::App;
#[macro_use]
extern crate glium;

fn main() {
	run_app();
}

fn run_app() {
	let rect = Rect::new(50, 150, 400, 150,rgb(0, 0, 0));
	let rect2 = Rect::new(50, 150, 400, 150,rgb(0, 200, 20));
	
	/// FIXME not sure why its not working
	let container = Container::new(rect);


	let vstack = VStack::new(16, vec![
		Box::new(container),
		Box::new(rect2)
	]).colour(rgb(25, 25, 125));

	let page = View::new(vstack);

	let app = 
		App::new()
		.add_view(page);

	app.run()
}

