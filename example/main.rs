use helium::{
    app::{events::{Event,EventQueue, UserEvent}, view::View, App}, hex, hstack, widgets::Button
};

fn main() {
    env_logger::init();
	app();
}

fn app(){
	let mut event_loop = EventQueue::new();
	// FIXME nested events not working
	let button = Button::new("Hello world")
		.padding(12)
		.on_click(&mut event_loop, ||{println!("Hello")})
		.color(hex!("#aaabbb"));

	let hstack = hstack![button].spacing(10);
	dbg!(&event_loop);

	let page = View::new(hstack,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}