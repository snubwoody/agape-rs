use helium::{
    app::{view::View, App}, hex, hstack, widgets::{Button, Text}
};

fn main() {
    env_logger::init();
	app();
}

fn app(){
	let color = hex!("#afffff");
	
	let hstack = hstack![
		Text::new("Hello world"),
		Text::new("1")
	].spacing(54).padding(24);
    
	let page = View::new(hstack);
    let app = App::new().add_view(page);
    
	app.run();
}