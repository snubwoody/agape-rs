use helium::{
    app::{view::View, App}, hex, hstack, widgets::{Button, Rect, Text}
};

fn main() {
    env_logger::init();
	app();
}

fn app(){
	let color = hex!("#afffff");
	
	let hstack = hstack![
		Text::new("Hello wo"),
		Text::new("1"),
		Rect::new(200.0,200.0,color)
	].spacing(54).padding(24);
    
	let page = View::new(hstack);
    let app = App::new().add_view(page);
    
	app.run();
}