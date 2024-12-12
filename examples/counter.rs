use helium::{
    app::{events::EventQueue, view::View, App}, hex, hstack, signal::Signal, widgets::{Button, Circle, Rect, Text}, BLUE, RED
};

fn main() {
    env_logger::init();
	app();
}

/// Broken
fn app(){
	let event_loop = EventQueue::new();

	let hstack = hstack![
		Button::new("Click me").color(hex!("#fababa")),
		Button::new("Click me").color(hex!("#fababa")),
		Button::new("Click me").color(hex!("#fababa"))
	].padding(24).spacing(56);
	
	let page = View::new(hstack,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}