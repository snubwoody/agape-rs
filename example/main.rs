use helium::{
    app::{view::View, App}, {BLUE, GREEN, TEAL}, hstack, widgets::Rect
};

fn main() {
    env_logger::init();
	
	let hstack = hstack![
		Rect::new(200.0, 150.0, GREEN),
		Rect::new(100.0, 250.0, TEAL),
		Rect::new(250.0, 250.0, BLUE)
	].spacing(54).padding(24);
    
	let page = View::new(hstack);
    let app = App::new().add_view(page);
    
	app.run();
}
