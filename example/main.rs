use std::future::poll_fn;

use rustui::{
    app::{view::View, App}, colour::{BLACK, BLUE, GREEN}, hstack, vstack, widgets::{button::Button, stack::Stack, text::Text}
};

#[tokio::main]
async fn main() {
    new_app();
}

async fn new_app() {
	let button = 
		Button::new("Click me")
		.padding(12)
		.colour(GREEN);

	let hstack = hstack!{
		Text::new("Hello"),
		Text::new("World")
	};

	let vstack = vstack!{
		spacing:20,
		padding:10,
		Button::new("Click me").colour(GREEN).padding(12),
		Button::new("Click me").colour(BLUE).padding(52),
		hstack
	};

    let page = View::new(vstack);
    let app = App::new().await.add_view(page);
    app.run().await;
}
