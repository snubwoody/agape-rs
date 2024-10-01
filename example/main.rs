use rustui::{
    app::{view::View, App},
    colour::{AMBER, BLACK, GREEN, INDIGO, PINK, TEAL},
    widgets::{container::Container, rect::Rect, stack::{HStack, VStack}, text::Text, Widget, WidgetTree
	},
};

fn main() {
    new_app();
}

fn new_app() {
	let text = Text::new("Hello world");

    let page = View::new(text);
    let app = App::new().add_view(page);
    app.run();
}
