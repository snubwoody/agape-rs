mod widgets;
mod view;
mod colour;
mod app;
pub mod utils;
pub mod surface;
pub mod text;
pub mod vertex;
pub mod layout;
use colour::Colour;
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
	let rect = Rect::new(50, 150, 400, 150,Colour::Rgb(0, 20, 200));

	let page = View::new(rect);

	let app = 
		App::new()
		.add_view(page);

	app.run()
}

