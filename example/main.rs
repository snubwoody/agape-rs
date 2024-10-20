use helium::{
    app::{view::View, App}, 
	color::TEAL, 
	widgets::{Rect}
};

fn main() {
    env_logger::init();
    new_app()
}

fn new_app() {
	let rect = 
		Rect::new(200.0, 150.0, TEAL).on_press(|_|{dbg!("I was pressed");});
    
	let page = View::new(rect);
    let app = App::new().add_view(page);
    app.run();
}
