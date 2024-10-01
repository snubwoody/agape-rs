use rustui::{
    app::{view::View, App},
    colour::{AMBER, BLACK, GREEN, INDIGO, PINK, TEAL},
    widgets::{button::Button, container::Container, rect::Rect, stack::{HStack, VStack}, text::Text, Widget, WidgetTree
	},
};

fn main() {
    new_app();
}

fn new_app() {
	let button = 
		Button::new("Click me")
		.padding(12)
		.colour(GREEN)
		.width(120.0)
		.height(40.0);

    let page = View::new(button);
    let app = App::new().add_view(page);
    app.run();
}
