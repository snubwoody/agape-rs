use rustui::{
	app::{view::View, App}, colour::{self, Colour, BLACK, GREEN, PINK, WHITE}, convert, view, widgets::{
		button::Button, container::{self, Container}, rect::Rect, stack::HStack, Edge, Node, Widget, WidgetTree
	}
};
use properties::{graph,page};


fn main() {
	new_app();
}

fn new_app(){
	let rect1 = Rect::new(200, 150, WHITE);
	let rect2 = Rect::new(150,200,WHITE);
	let dummy = Rect::new(150,200,WHITE);
	let hstack = HStack{
		spacing:0,
		padding:0,
		colour:BLACK,
		children:vec![
			Box::new(rect1),
			Box::new(rect2)
		]
	};
	
	let mut graph = WidgetTree::new();
	graph.build(hstack);
	dbg!(&graph);
	let button = Node{
		id:4,
		body:Button::new("Hi").colour(PINK).build(),
		edges:vec![Edge::Parent(5)],
	};
	let container = Node{
		id:5,
		body:Container::new(dummy).colour(BLACK).padding(12).build(),
		edges:vec![Edge::Child(4)],
	};
	
	graph.add(container);
	graph.add(button);
	graph.root(5);
}






