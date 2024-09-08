use rustui::{
	app::{view::View, App}, 
	colour::Colour,
	widgets::{
		rect::Rect, 
		stack::HStack, 
		text::Text, WidgetTree
	}
};

fn main() {
	new_app();
}

fn new_app(){
	let mut widget_tree = WidgetTree::new();
	let greeting = Text::new("Hello world");
	let _box = Rect::new(200, 150, Colour::Rgb(244, 144, 244));
	let hstack = HStack{
		spacing:12,
		padding:0,
		children:vec![]
	};

	widget_tree.add(hstack, 0, None,vec![1,2]);
	widget_tree.add(greeting, 1, Some(0),vec![]);
	widget_tree.add(_box, 2, Some(0),vec![]);

	let home = View::new(widget_tree);

	let app = 
		App::new()
		.add_view(home);
	app.run();
}


