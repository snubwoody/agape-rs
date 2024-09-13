use rustui::{
	app::{view::View, App}, 
	colour::Colour,
	widgets::{
		container::Container, rect::Rect, stack::{HStack, VStack}, text::Text, WidgetTree
	}
};

fn main() {
	new_app();
}

fn new_app(){
	let _box = Rect::new(200, 150, Colour::Rgb(244, 144, 244));
	let _box2 = Rect::new(10, 55, Colour::Rgb(244, 1, 99));
	let _box3 = Rect::new(150, 25, Colour::Rgb(44, 1, 99));
	let _box4 = Rect::new(100, 50, Colour::Rgb(144, 10, 199));
	let _box5 = Rect::new(100, 50, Colour::Rgb(44, 10, 199));
	let container = Container::new(_box5);

	let column = VStack{
		spacing:32,
		padding:0,
		children:vec![
			Box::new(_box3),
			Box::new(_box4),
		]
	};

	let hstack = HStack{
		spacing:12,
		padding:0,
		children:vec![
			Box::new(_box2),
			Box::new(container),
			Box::new(_box),
			Box::new(column)
		]
	};
	let widget_tree = WidgetTree::new(hstack);
	//dbg!(&widget_tree);

	let home = View::new(widget_tree);

	let app = 
		App::new()
		.add_view(home);
	app.run();
}


