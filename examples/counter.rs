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

	let hstack = hstack![
		Container::new(Text::new("Hello world")),
		Text::new("Hello world")
	].padding(24).spacing(56).color(RED);
	
	let page = View::new(hstack,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}