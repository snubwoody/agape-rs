use helium::{
    app::{events::{Event,EventQueue, UserEvent}, view::View, App}, hex, hstack, widgets::{Button, Circle, Rect}, BLUE
};

fn main() {
    env_logger::init();
	app();
}

fn app(){
	let event_loop = EventQueue::new();

	let circle = Circle::new(200, hex!("#000000"));
	let rect = Rect::new(200, 200, BLUE);

	let page = View::new(circle,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}