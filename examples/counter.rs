use helium::{
    app::{events::EventQueue, view::View, App}, hex, hstack, 
	widgets::{Button,Rect, Text}, BLACK,
};

fn main() {
    env_logger::init();
	app();
}

/// Broken
fn app(){
	let event_loop = EventQueue::new();

	let hstack = hstack!{
		Button::new("Hello world").padding(12).corner_radius(8),
		Text::new("Hello world"),
		Button::new("Hello world").padding(12).corner_radius(8),
		Text::new("Hello world")
	};
	
	let page = View::new(hstack,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}