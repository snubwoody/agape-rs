use rustui::{
	app::{view::View, App}, 
	widgets::{image::Image, text::Text}
};

fn main() {
	let greeting = Text::new("Hello world");
	let image = Image{
		path:"example/images/Frank Ocean Blonde.jpg".to_owned(),
		width:500,
		height:500
	};
	let home = View::new(image);
	let app = 
		App::new()
		.add_view(home);
	app.run();
}