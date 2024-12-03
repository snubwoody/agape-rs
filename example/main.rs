use helium::{
    app::{view::View, App}, hex, hstack, vstack, widgets::{Button, Container, Rect, Text, Widget}, BLACK
};

fn main() {
    env_logger::init();
	app();
}

fn app(){
	// FIXME somehow has a block layout
	let button = Button::new("Hello world")
		.tap(||{println!("I was tapped")})
		.padding(12)
		.color(hex!("#aaabbb"));

	let page = View::new(button);
  	let app = App::new().add_view(page);
    
	app.run();
}