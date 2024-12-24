use helium::{
    app::{events::EventQueue, view::View, App}, hex, hstack, 
	widgets::{Button, Container, Rect, Text}, BLACK, BLUE, RED
};

fn main() {
    env_logger::init();
	app();
}

/// Broken
fn app(){
	let mut event_loop = EventQueue::new();

	let hstack = hstack!{
		Button::new("Hello world"),
		Button::new("Hello world"),
		Button::new("Hello world"),
		Button::new("Hello world"),
		Rect::new(200.0, 200.0, BLACK)
	};
	
	let page = View::new(hstack,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}