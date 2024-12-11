use helium::{
    app::{events::EventQueue, view::View, App}, hstack, widgets::{Circle, Rect}, BLUE, RED
};

fn main() {
    env_logger::init();
	app();
}

fn app(){
	let event_loop = EventQueue::new();

	let circle = Circle::new(150, RED);
	let rect = Rect::new(150, 150, BLUE).radius(10);

	let page = View::new(hstack![circle,rect].spacing(24).padding(56),event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}