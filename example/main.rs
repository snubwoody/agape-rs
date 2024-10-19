use helium::{
    app::{view::View, App}, 
 
	hstack, 
	widgets::{Button, Rect, Text}
};

fn main() {
    env_logger::init();
    new_app()
}

fn new_app() {
	let hstack = hstack![Text::new("Hello world"),Button::new("Hello world")];
    let page = View::new(hstack);
    let app = App::new().add_view(page);
    app.run();
}
