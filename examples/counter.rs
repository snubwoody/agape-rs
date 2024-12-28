use helium::{
    app::{events::EventQueue, view::View, App}, hex, hstack, widgets::{Button, Circle, Image, Rect, Text}, BoxSizing, BLACK
};

fn main() {
    env_logger::init();
	app();
}

/// Broken
fn app(){
	let event_loop = EventQueue::new();

	let image = Image{};
	let page = View::new(image,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}