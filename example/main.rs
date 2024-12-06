use helium::{
    app::{events::{Event,EventQueue, UserEvent}, view::View, App}, hex, 
	widgets::Button,
};

fn main() {
    env_logger::init();
	app();
}

fn app(){
	let mut event_loop = EventQueue::new();
	// FIXME somehow has a block layout
	let button = Button::new("Hello world")
		.padding(12)
		.color(hex!("#aaabbb"));

	event_loop.push(UserEvent::new(
		button.get_id(), 
		Event::OnClick(Box::new(||{println!("Heyy!!")}))
	));
	
	let page = View::new(button,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}