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
	let rect_1 = Rect::new(200.0, 50.0, BLACK);
	let rect_2 = Rect::new(300.0, 150.0, BLACK);

	let vstack = VStack{
		padding:12,
		spacing:24,
		colour:GREEN,
		children: vec![
			Box::new(rect_1),
			Box::new(rect_2),
		]
	};

    let page = View::new(vstack);
    let app = App::new().add_view(page);
    app.run();
}
