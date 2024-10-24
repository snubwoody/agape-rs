use helium::{
    app::{view::View, App}, color::BLUE, widgets::{Button, Rect, Text}
};

fn main() {
    env_logger::init();
    new_app()
}

fn new_app() {
	let button = Button::new("Click me");
    
	let page = View::new(button);
    let app = App::new().add_view(page);
    app.run();
}
