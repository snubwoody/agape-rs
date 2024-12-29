use helium::{
    app::{events::EventQueue, view::View, App}, 
	hex, hstack, 
	widgets::{Button, Circle, Image, ImageSource, Rect, Text}, BoxSizing, BLACK
};

fn main() {
    env_logger::init();
	app();
}

/// Broken
fn app(){
	let event_loop = EventQueue::new();

	let path = "c:/Users/wakun/Projects/Tools/Rust-UI/helium/icons/menu.png";
	let path = "c:/Users/wakun/Projects/Tools/Rust-UI/examples/temp/image.jpg";
	let path = "C:/Users/wakun/Projects/Tools/Rust-UI/examples/temp/legends never die.png";

	let image = Image::file(path);
	let page = View::new(image,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}