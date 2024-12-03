use helium::{
    app::{view::View, App}, hex, hstack, vstack, widgets::{Button, Container, Rect, Text, Widget}, BLACK
};

fn main() {
    env_logger::init();
	app();
}

fn app(){
	// FIXME somehow has a block layout
	let color = hex!("#afffff");
	let container = Container::new(
		Rect::new(200, 200, color.clone())
	).padding(44).color(BLACK);

	let page = View::new(container);
    let app = App::new().add_view(page);
    
	app.run();
}