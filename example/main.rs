use rustui::{
	app::{view::View, App}, 
	colour::{self, Colour, BLACK, GREEN, PINK, WHITE},
	widgets::{
		button::Button, rect::Rect, stack::HStack, Edge, Node, Widget, WidgetTree
	}
};


fn main() {
	new_app();
}

fn new_app(){
	let _rect1_ = Rect::new(200, 150, WHITE);
	let _rect2_ = Rect::new(150,300,WHITE);
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
		edges:vec![Edge::Parent(3),Edge::Sibling(1),Edge::Sibling(0)],
	};
	let rect1 = Node{
		id:0,
		body:_rect1_.build(),
		edges:vec![Edge::Parent(3),Edge::Sibling(1),Edge::Sibling(4)]
	};
	let rect2 = Node{
		id:1,
		body:_rect2_.build(),
		edges:vec![Edge::Parent(3),Edge::Sibling(0),Edge::Sibling(4)]
	};
	let hstack = Node{
		id:3,
		body:_hstack_.build(),
		edges:vec![Edge::Child(0),Edge::Child(1),Edge::Child(4)]
	};
	

	graph.add(hstack);
	graph.add(rect1);
	graph.add(rect2);
	graph.add(button);
	graph.root(3);
	//dbg!(&graph);

	let page = View::new(graph);
	let app = 
		App::new()
		.add_view(page);

	app.run();
}






