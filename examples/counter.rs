use helium::{
    app::{events::EventQueue, view::View, App}, hstack, widgets::{Image, Text}
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
	let svg_path = "C:/Users/wakun/Projects/Tools/Rust-UI/helium/icons/feather-icons/airplay.svg";
	let svg_path_2 = "C:/Users/wakun/Projects/Tools/Rust-UI/helium/icons/feather-icons/alert-octagon.svg";

	let main = hstack!{
		Text::new("Hello world"),
		Image::file(path),
		Text::new("Hello world"),
		Image::svg(svg_path),
		Text::new("Hello world"),
		Image::svg(svg_path_2),
		Text::new("Hello world")
	};

	let page = View::new(main,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}