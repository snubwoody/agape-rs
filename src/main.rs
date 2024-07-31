mod widgets;
mod view;
mod colour;
mod app;
pub mod surface;
pub mod text;
pub mod vertex;
use colour::rgb;
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
	let rect2 = Rect::new(50, 150, 400, 150,rgb(0, 0, 0));
	let rect3 = Rect::new(50, 150, 400, 150,rgb(0, 0, 0));
	let rect4 = Rect::new(50, 150, 400, 150,rgb(20, 40, 0));
	let rect5 = Rect::new(50, 150, 400, 150,rgb(0, 20, 40));
	
	let vstack = VStack::new(12, vec![
		Box::new(rect),
		Box::new(rect2),
		Box::new(rect3)]
	).colour(rgb(23, 119, 122));

	let hstack = HStack::new(12,vec![
		Box::new(rect4),
		Box::new(rect5),
		Box::new(vstack)
	]).colour(rgb(155, 155, 155));

	let page = View::new(hstack);

	let app = 
		App::new()
		.add_view(page);

	app.run()
}

/// Map value from one range to another. Any overflow is clipped to the min or max
fn map(mut value:f32,input_range:[f32;2],output_range:[f32;2]) -> f32{
	if value > input_range[1]{
		value = input_range[1]
	}
	else if value < input_range[0] {
		value = input_range[0]
	}

	let scale = (output_range[1] - output_range[0]) / (input_range[1] - input_range[0]);
	let offset = input_range[0]*(scale)+output_range[0];

	return  value * scale + offset;
}

