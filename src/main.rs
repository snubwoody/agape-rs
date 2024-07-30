mod widgets;
mod view;
mod colour;
mod app;
pub mod layout;
pub mod surface;
pub mod text;
pub mod vertex;
use colour::rgb;
use layout::Layout;
use widgets::container::Container;
use widgets::rect::Rect;
use widgets::text::Text;
use crate::surface::Surface;
use crate::widgets::Widget;
use crate::view::View;
use crate::app::App;
#[macro_use]
extern crate glium;


fn main() {
	run_app();
}

fn run_app<'a>() {
	let rect = Rect::new(50, 150, 400, 500, rgb(234, 104, 34));
	let container = Container::new(rect);
	let page = View::new(container);

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

