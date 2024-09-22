use rustui::{
	app::{view::View, App}, 
	colour::{self, Colour, BLACK, GREEN, PINK, WHITE},
	widgets::{
		button::Button, container::{self, Container}, rect::Rect, stack::HStack, Edge, Node, Widget, WidgetTree
	}
};


fn main() {
	new_app();
}

fn new_app(){
	let _rect1_ = Rect::new(200, 150, WHITE);
	let _rect2_ = Rect::new(150,200,WHITE);
	let dummy = Rect::new(150,200,WHITE);
	let _hstack_ = HStack{
		padding:24,
		spacing:36,
		colour:BLACK,
		children:vec![]
	};

	let mut graph = WidgetTree::new();
	
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
	//dbg!(&graph);

	let page = View::new(graph);
	let app = 
		App::new()
		.add_view(page);

	app.run();
}






