use helium::{
    app::{events::EventQueue, view::View, App}, hex, hstack, widgets::{Button, Circle, Rect, Text}, BLUE, RED
};

fn main() {
    env_logger::init();
	app();
}

fn app(){
	let event_loop = EventQueue::new();

	let count = Text::new("1");
	let decrement = Button::new("Decrement").color(hex!("#0f0f0f"));
	let increment = Button::new("Increment").color(hex!("#0f0f0f"));
	let hstack = hstack![decrement,count,increment].spacing(24).padding(12);

	let page = View::new(hstack,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}