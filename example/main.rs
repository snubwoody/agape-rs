use helium::{
    app::{view::View, App}, 
	colour::{self}, 
	hstack, 
	widgets::{Rect, Text}
};

fn main() {
    env_logger::init();
    new_app()
}

fn new_app() {
	let text = Text::new("Hello world");
    let page = View::new(text);
    let app = App::new().add_view(page);
    app.run();
}
