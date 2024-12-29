use helium::{
    app::{events::EventQueue, view::View, App}, hstack, widgets::{icon::feather_icons, Image, Text, Widget}, LayoutSolver, Size
};

fn main() {
    env_logger::init();
	app();
}

fn app(){
	let event_loop = EventQueue::new();

	let path = "c:/Users/wakun/Projects/Tools/Rust-UI/helium/icons/menu.png";
	let path = "c:/Users/wakun/Projects/Tools/Rust-UI/examples/temp/image.jpg";
	let path = "C:/Users/wakun/Projects/Tools/Rust-UI/examples/temp/legends never die.png";
	let svg_path = "C:/Users/wakun/Projects/Tools/Rust-UI/helium/icons/feather-icons/airplay.svg";
	let svg_path_2 = "C:/Users/wakun/Projects/Tools/Rust-UI/helium/icons/feather-icons/alert-octagon.svg";

	let main = hstack!{
		feather_icons::airplay(),
		feather_icons::_box(),
		feather_icons::facebook()
	};

	let page = View::new(main,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}

