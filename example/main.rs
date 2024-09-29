use properties::{graph, page};
use rustui::{
    app::{view::View, App},
    colour::{BLACK, GREEN, INDIGO, PINK, TEAL},
    widgets::{container::Container, rect::Rect, stack::HStack, Node, Widget, WidgetTree
	},
};

fn main() {
    new_app();
}

fn new_app() {
    let rect1 = Rect::new(200.0, 150.0, INDIGO);
    let rect2 = Rect::new(150.0, 200.0, PINK);

    let dummy = Container::new(Rect::new(150.0, 200.0, GREEN)).colour(TEAL);
    let hstack = HStack {
        spacing: 54,
        padding: 10,
        colour: BLACK,
        children: vec![Box::new(rect1), Box::new(rect2), Box::new(dummy)],
    };

    let mut graph = WidgetTree::new(hstack.build());
    dbg!(&graph);

    let page = View::new(graph);
    let app = App::new().add_view(page);
    app.run();
}
