use rustui::{
    app::{view::View, App}, colour::{BLACK, BLUE, GREEN}, vstack, widgets::{button::Button, stack::Stack}
};

fn main() {
    new_app();
}

fn new_app() {
	let button = 
		Button::new("Click me")
		.padding(12)
		.colour(GREEN);

	let vstack = vstack!{
		Button::new("Click me").colour(GREEN).padding(12),
		Button::new("Click me").colour(BLUE).padding(52)
	};

    let page = View::new(vstack);
    let app = App::new().add_view(page);
    app.run();
}
