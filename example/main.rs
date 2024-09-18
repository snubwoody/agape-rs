use rustui::{
	app::{view::View, App}, 
	colour::{self, Colour},
	widgets::{
		stack::{HStack, VStack}, 
		text::Text,
		Rect, 
		Widget, 
		WidgetNode, 
		WidgetTree
	}
};

macro_rules! view {
	($widget:ident{$($field:ident: $value:expr,)*}) => {
		rustui::widgets::$widget{
			$($field: $value,)*
		}
	};
}


fn main() {
	let k = view!{
		Rect{
			width:200,
			height:150,
			colour:Colour::Rgb(255,240,242),
		}
	};

	let k = stringify!("").to_lowercase();

	//new_app();
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

	


	let widget_tree = WidgetTree::new().build(hstack);
	//dbg!(&widget_tree);

	let home = View::new(widget_tree);

	let app = 
		App::new()
		.add_view(home);
	app.run();
}




