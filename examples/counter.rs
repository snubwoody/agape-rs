use helium::{
    app::{events::EventQueue, view::View, App}, hex, hstack, 
	widgets::{Button,Rect}, BLACK,
};

fn main() {
    env_logger::init();
	app();
}

/// Broken
fn app(){
	let event_loop = EventQueue::new();

	let hstack = hstack!{
		Button::new("Hello world").padding(24),
		Button::new("Hello world").padding(24),
		Button::new("Hello world").padding(24),
		Button::new("Hello world").padding(24),
		Rect::new(200.0, 200.0, BLACK)
	};
	
	let page = View::new(hstack,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}