use helium::{
    app::{events::EventQueue, view::View, App}, hstack, widgets::{icon::feather_icons, Image, Text, Widget}, LayoutSolver, Size
};

fn main() {
    env_logger::init();
	app();
}

fn app(){
	let event_loop = EventQueue::new();

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

