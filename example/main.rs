use helium::{
    app::{view::View, App}, color::TEAL, hstack, vstack, widgets::{Button, Rect, Text, Widget}
};

fn main() {
    env_logger::init();
    new_app()
}

fn new_app() {
	let rect = 
		Rect::new(200.0, 150.0, TEAL)
		.on_hover(|rect|{});
    
	let page = View::new(rect);
    let app = App::new().add_view(page);
    app.run();
}
