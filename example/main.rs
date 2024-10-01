use rustui::{
    app::{view::View, App},
    colour::{AMBER, BLACK, GREEN, INDIGO, PINK, TEAL},
    widgets::{container::Container, rect::Rect, stack::HStack, Widget, WidgetTree
	},
};

fn main() {
    new_app();
}

fn new_app() {

    let dummy = Container::new(Rect::new(150.0, 200.0, GREEN)).colour(AMBER).padding(12);

    let page = View::new(dummy);
    let app = App::new().add_view(page);
    app.run();
}
