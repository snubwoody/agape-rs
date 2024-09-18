use rustui::{
	app::{view::View, App}, 
	colour::{self, Colour},
	widgets::{
		container::Container, 
		rect::Rect, 
		stack::{HStack, VStack}, 
		text::Text, 
		Widget, 
		WidgetNode, 
		WidgetTree
	}
};

macro_rules! view {
	($widget:expr) => {
		let widget = $widget;
		return rustui::widgets::container::widget{}
	};
}

fn main() {
	let k = view!{
		Container{
			child:Box::new(Rect::new(200, 100, Colour::Rgb(255, 10, 100))),
			padding:20,
			colour:Colour::Rgb(255, 255, 255)
		}
	};
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




