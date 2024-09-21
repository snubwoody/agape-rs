use rustui::{
	app::{view::View, App}, 
	colour::{self, Colour, BLACK, GREEN, PINK},
	widgets::{
		rect::Rect, 
		stack::HStack, 
		Edge, 
		Node,
		Widget, 
		WidgetTree
	}
};
use properties::view;


fn main() {
	new_app();
}

fn new_app(){
	let _rect1_ = Rect::new(200, 150, BLACK);
	let _rect2_ = Rect::new(150,300,GREEN);
	let _hstack_ = HStack{
		padding:0,
		spacing:12,
		children:vec![]
	};

	let mut graph = WidgetTree::new();
	let rect1 = Node{
		id:0,
		body:_rect1_.build(),
		edges:vec![Edge::Parent(3),Edge::Sibling(1)]
	};
	let rect2 = Node{
		id:1,
		body:_rect2_.build(),
		edges:vec![Edge::Parent(3),Edge::Sibling(0)]
	};
	let hstack = Node{
		id:3,
		body:_hstack_.build(),
		edges:vec![Edge::Child(0),Edge::Child(1)]
	};
	

	graph.add(rect1);
	graph.add(rect2);
	graph.add(hstack);
	//dbg!(&graph);

	let page = View::new(graph);
	let app = 
		App::new()
		.add_view(page);

	app.run();
}






