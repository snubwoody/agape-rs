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
	let greeting = Text::new("Hello world");
	let _box = Rect::new(200, 150, Colour::Rgb(244, 144, 244));
	let hstack = HStack{
		spacing:12,
		padding:0,
		children:vec![
			Box::new(greeting),
			Box::new(_box)
		]
	};
	let widget_tree = WidgetTree::new(hstack);

	let home = View::new(widget_tree);

	let app = 
		App::new()
		.add_view(home);
	app.run();
}


