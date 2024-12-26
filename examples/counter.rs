use helium::{
    app::{events::EventQueue, view::View, App}, hex, hstack, widgets::{Button,Rect, Text}, BoxSizing, BLACK
};

fn main() {
    env_logger::init();
	app();
}

/// Broken
fn app(){
	let event_loop = EventQueue::new();


	let mut hstack = hstack!{
		Rect::new(200.0, 200.0, BLACK),
		Rect::new(200.0, 200.0, BLACK).flex_width(1),
		Rect::new(200.0, 200.0, BLACK).flex_width(2)
	}.spacing(12).padding(24);

	hstack.intrinsic_size.width = BoxSizing::Flex(1);
	hstack.intrinsic_size.height = BoxSizing::Flex(2);
	
	let page = View::new(hstack,event_loop);
  	
	App::new()
	.add_view(page)
	.run();
}