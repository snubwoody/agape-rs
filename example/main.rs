use helium::{
    app::{events::EventQueue, view::View, App}, widgets::{Circle, Rect}, BLUE, RED
};

fn main() {
    env_logger::init();
	app();
}

fn app(){
	let event_loop = EventQueue::new();

	let circle = Circle::new(50, RED);
	let rect = Rect::new(200, 200, BLUE);

	let page = View::new(circle,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}