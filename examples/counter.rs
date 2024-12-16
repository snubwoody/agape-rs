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
	let event_loop = EventQueue::new();

	let button = Button::new("Hello world");
	
	let page = View::new(button,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}