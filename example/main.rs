use rustui::{
	app::{view::View, App}, 
	colour::{self, Colour, BLACK, GREEN, PINK},
	widgets::{
		rect::Rect, stack::HStack, Widget, WidgetNode, WidgetTree
	}
};
use properties::view;


fn main() {
	let _box = Rect::new(200, 150, BLACK);
	let rect = Rect::new(150,300,GREEN);
	let rect2 = Rect::new(150,300,PINK);
	let hstack = HStack{
		padding:0,
		spacing:12,
		children:vec![]
	};

	let mut tree = WidgetTree::new();
	tree.add(hstack, None, vec![1,2,3],0);
	tree.add(_box, Some(0), vec![],1);
	tree.add(rect, Some(0), vec![],2);
	tree.add(rect2, Some(0), vec![],3);

	let home = View::new(tree);

	let app = 
		App::new()
		.add_view(home);
	app.run(); 
}





