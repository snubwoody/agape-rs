use properties::{graph, page};
use rustui::{
    app::{view::View, App},
    colour::{BLACK, GREEN, INDIGO, PINK, TEAL, WHITE},
    widgets::{
        button::Button, container::Container, rect::Rect, stack::HStack, Edge, Node, Widget,
        WidgetTree,
    },
};

fn main() {
    new_app();
}

fn new_app() {
    let rect1 = Rect::new(200, 150, INDIGO);
    let rect2 = Rect::new(150, 200, PINK);
    let dummy = Container::new(Rect::new(150, 200, GREEN)).colour(TEAL);
    let hstack = HStack {
        spacing: 54,
        padding: 10,
        colour: BLACK,
        children: vec![Box::new(rect1), Box::new(rect2),Box::new(dummy)],
    };

    let mut graph = WidgetTree::new();
    graph.build(hstack);
    //dbg!(&graph);
   
    let page = View::new(graph);
    let app = App::new().add_view(page);
    app.run();
}
