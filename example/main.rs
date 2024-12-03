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
	let button = Button::new("Hello world").on_click(||println!("Hello world"));

	let page = View::new(button);
  	let app = App::new().add_view(page);
    
	app.run();
}