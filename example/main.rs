use rustui::{
	app::{view::View, App}, 
	widgets::text::Text
};

fn main() {
	app()
}

fn app(){
	let greeting = Text::new("Hello world");
	
	let home = View::new(greeting);
	let app = 
		App::new()
		.add_view(home);
	app.run();
}

